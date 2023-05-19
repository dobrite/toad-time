#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

mod display;
mod screens;
mod state;

const MICRO_SECONDS_PER_SECOND: u32 = 1_000_000;
pub type MicroSeconds = fugit::Duration<u64, 1, MICRO_SECONDS_PER_SECOND>;

#[rtic::app(
    device = rp_pico::hal::pac,
    dispatchers = [TIMER_IRQ_1, TIMER_IRQ_2]
)]
mod app {
    use defmt_rtt as _;
    use embedded_hal::{
        digital::v2::{InputPin, OutputPin, ToggleableOutputPin},
        spi,
    };
    use fugit::RateExtU32;
    use panic_probe as _;
    use rotary_encoder_embedded::{standard::StandardMode, Direction, RotaryEncoder};
    use rp_pico::{
        hal::{
            self, clocks,
            gpio::{
                pin::{bank0::*, FunctionSpi, PullUp, PushPullOutput},
                Input, Pin,
            },
            sio::Sio,
            spi::Spi,
            watchdog::Watchdog,
            Clock,
        },
        XOSC_CRYSTAL_FREQ,
    };
    use rtic_monotonics::rp2040::{Timer, *};
    use rtic_sync::{channel::*, make_channel};
    use seq::Seq;
    use ssd1306::{prelude::*, Ssd1306};

    use super::{
        display::Display,
        screens::Screens,
        state::{Command, Gate, PlayStatus, State, StateChange},
        MicroSeconds,
    };

    const COMMAND_CAPACITY: usize = 4;
    const STATE_CHANGE_CAPACITY: usize = 4;

    type Encoder =
        RotaryEncoder<StandardMode, Pin<Gpio14, Input<PullUp>>, Pin<Gpio15, Input<PullUp>>>;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        display: Display,
        encoder: Encoder,
        encoder_button: Pin<Gpio13, Input<PullUp>>,
        gate_a: Pin<Gpio2, PushPullOutput>,
        gate_b: Pin<Gpio3, PushPullOutput>,
        gate_c: Pin<Gpio4, PushPullOutput>,
        gate_d: Pin<Gpio5, PushPullOutput>,
        play_button: Pin<Gpio11, Input<PullUp>>,
        page_button: Pin<Gpio12, Input<PullUp>>,
    }

    #[init()]
    fn init(mut ctx: init::Context) -> (Shared, Local) {
        unsafe {
            hal::sio::spinlock_reset();
        }

        let token = rtic_monotonics::create_rp2040_monotonic_token!();
        Timer::start(ctx.device.TIMER, &mut ctx.device.RESETS, token);
        let mut watchdog = Watchdog::new(ctx.device.WATCHDOG);
        let clocks = clocks::init_clocks_and_plls(
            XOSC_CRYSTAL_FREQ,
            ctx.device.XOSC,
            ctx.device.CLOCKS,
            ctx.device.PLL_SYS,
            ctx.device.PLL_USB,
            &mut ctx.device.RESETS,
            &mut watchdog,
        )
        .ok()
        .unwrap();

        let mut delay =
            cortex_m::delay::Delay::new(ctx.core.SYST, clocks.system_clock.freq().to_Hz());

        let sio = Sio::new(ctx.device.SIO);
        let pins = rp_pico::Pins::new(
            ctx.device.IO_BANK0,
            ctx.device.PADS_BANK0,
            sio.gpio_bank0,
            &mut ctx.device.RESETS,
        );

        let display = {
            let oled_dc = pins.gpio16.into_push_pull_output();
            let oled_cs = pins.gpio17.into_push_pull_output();
            let _ = pins.gpio18.into_mode::<FunctionSpi>();
            let _ = pins.gpio19.into_mode::<FunctionSpi>();
            let mut oled_reset = pins.gpio20.into_push_pull_output();

            let spi = Spi::new(ctx.device.SPI0).init(
                &mut ctx.device.RESETS,
                125_000_000u32.Hz(),
                1_000_000u32.Hz(),
                &spi::MODE_0,
            );

            let interface = SPIInterface::new(spi, oled_dc, oled_cs);
            let mut display_ctx =
                Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
                    .into_buffered_graphics_mode();

            display_ctx.reset(&mut oled_reset, &mut delay).unwrap();
            display_ctx.init().unwrap();

            Display::new(display_ctx)
        };

        let play_button = pins.gpio11.into_pull_up_input();
        let page_button = pins.gpio12.into_pull_up_input();
        let encoder_button = pins.gpio13.into_pull_up_input();
        let rotary_dt = pins.gpio14.into_pull_up_input();
        let rotary_clk = pins.gpio15.into_pull_up_input();
        let encoder = RotaryEncoder::new(rotary_dt, rotary_clk).into_standard_mode();

        let (command_sender, command_receiver) = make_channel!(Command, COMMAND_CAPACITY);
        let (state_sender, tick_state_receiver) = make_channel!(StateChange, STATE_CHANGE_CAPACITY);
        let (tick_state_sender, display_state_receiver) =
            make_channel!(StateChange, STATE_CHANGE_CAPACITY);

        tick::spawn(tick_state_receiver, tick_state_sender).ok();
        state::spawn(command_receiver, state_sender).ok();
        display::spawn(display_state_receiver).ok();
        update_encoder::spawn(command_sender.clone()).ok();
        update_encoder_button::spawn(command_sender.clone()).ok();
        update_page_button::spawn(command_sender.clone()).ok();
        update_play_button::spawn(command_sender).ok();

        let mut gate_a = pins.gpio2.into_push_pull_output();
        let mut gate_b = pins.gpio3.into_push_pull_output();
        let mut gate_c = pins.gpio4.into_push_pull_output();
        let mut gate_d = pins.gpio5.into_push_pull_output();

        let _ = gate_a.set_high();
        let _ = gate_b.set_high();
        let _ = gate_c.set_high();
        let _ = gate_d.set_high();

        (
            Shared {},
            Local {
                display,
                encoder,
                encoder_button,
                gate_a,
                gate_b,
                gate_c,
                gate_d,
                page_button,
                play_button,
            },
        )
    }

    #[idle(local = [])]
    fn idle(_cx: idle::Context) -> ! {
        loop {
            cortex_m::asm::nop();
        }
    }

    #[task(local = [play_button], priority = 1)]
    async fn update_play_button(
        ctx: update_play_button::Context,
        command_sender: Sender<'static, Command, COMMAND_CAPACITY>,
    ) {
        debounced_button(command_sender, ctx.local.play_button, Command::PlayPress).await
    }

    #[task(local = [page_button], priority = 1)]
    async fn update_page_button(
        ctx: update_page_button::Context,

        command_sender: Sender<'static, Command, COMMAND_CAPACITY>,
    ) {
        debounced_button(command_sender, ctx.local.page_button, Command::PagePress).await
    }

    #[task(local = [encoder_button], priority = 1)]
    async fn update_encoder_button(
        ctx: update_encoder_button::Context,
        command_sender: Sender<'static, Command, COMMAND_CAPACITY>,
    ) {
        debounced_button(
            command_sender,
            ctx.local.encoder_button,
            Command::EncoderPress,
        )
        .await
    }

    async fn debounced_button<B: InputPin>(
        mut command_sender: Sender<'static, Command, COMMAND_CAPACITY>,
        button: &B,
        command: Command,
    ) where
        <B as InputPin>::Error: core::fmt::Debug,
    {
        let mut armed = true;
        let button_update_duration = MicroSeconds::from_ticks(50_000);

        loop {
            if armed && button.is_low().unwrap() {
                armed = false;
                let _ = command_sender.send(command).await;
            } else if !armed && button.is_high().unwrap() {
                armed = true;
            }

            Timer::delay(button_update_duration).await
        }
    }

    #[task(local = [encoder], priority = 1)]
    async fn update_encoder(
        ctx: update_encoder::Context,
        mut command_sender: Sender<'static, Command, COMMAND_CAPACITY>,
    ) {
        let encoder = ctx.local.encoder;
        let update_duration = MicroSeconds::from_ticks(1111);

        loop {
            encoder.update();
            match encoder.direction() {
                Direction::Clockwise => {
                    let _ = command_sender.send(Command::EncoderRight).await;
                }
                Direction::Anticlockwise => {
                    let _ = command_sender.send(Command::EncoderLeft).await;
                }
                Direction::None => {}
            }
            Timer::delay(update_duration).await
        }
    }

    #[task(local = [], priority = 1)]
    async fn state(
        _ctx: state::Context,
        mut command_receiver: Receiver<'static, Command, COMMAND_CAPACITY>,
        mut state_sender: Sender<'static, StateChange, STATE_CHANGE_CAPACITY>,
    ) {
        let mut state = State::new();

        while let Ok(command) = command_receiver.recv().await {
            match state.handle_command(command) {
                StateChange::None => {}
                state_change => {
                    state.handle_state_change(&state_change);
                    let _ = state_sender.send(state_change).await;
                }
            }
        }
    }

    #[task(local = [display], priority = 1)]
    async fn display(ctx: display::Context, mut state_receiver: Receiver<'static, StateChange, 4>) {
        let display = ctx.local.display;
        let mut screens = Screens::new();
        let mut state = State::new();
        screens.draw_home(&state, display);

        while let Ok(state_change) = state_receiver.recv().await {
            match state_change {
                StateChange::None => {}
                _ => state.handle_state_change(&state_change),
            }
            screens.draw(&state, &state_change, display);
        }
    }

    #[task(local = [gate_a, gate_b, gate_c, gate_d], priority = 2)]
    async fn tick(
        ctx: tick::Context,
        mut state_receiver: Receiver<'static, StateChange, STATE_CHANGE_CAPACITY>,
        mut state_sender: Sender<'static, StateChange, STATE_CHANGE_CAPACITY>,
    ) {
        let mut state = State::new();
        let mut seq = Seq::new(4);

        loop {
            let result = seq.tick();

            if result[0].edge_change {
                _ = ctx.local.gate_a.toggle();
            }

            if result[1].edge_change {
                _ = ctx.local.gate_b.toggle();
            }

            if result[2].edge_change {
                _ = ctx.local.gate_c.toggle();
            }

            if result[3].edge_change {
                _ = ctx.local.gate_d.toggle();
            }

            while let Ok(state_change) = state_receiver.try_recv() {
                state.handle_state_change(&state_change);
                match state_change {
                    StateChange::Rate(gate, rate) => match gate {
                        Gate::A => seq.set_rate(0, rate),
                        Gate::B => seq.set_rate(1, rate),
                        Gate::C => seq.set_rate(2, rate),
                        Gate::D => seq.set_rate(3, rate),
                    },
                    StateChange::Pwm(gate, pwm) => match gate {
                        Gate::A => seq.set_pwm(0, pwm),
                        Gate::B => seq.set_pwm(1, pwm),
                        Gate::C => seq.set_pwm(2, pwm),
                        Gate::D => seq.set_pwm(3, pwm),
                    },
                    StateChange::Prob(gate, prob) => match gate {
                        Gate::A => seq.set_prob(0, prob),
                        Gate::B => seq.set_prob(1, prob),
                        Gate::C => seq.set_prob(2, prob),
                        Gate::D => seq.set_prob(3, prob),
                    },
                    StateChange::PlayStatus(play_status) => match play_status {
                        PlayStatus::Playing => { /* TODO: pause */ }
                        PlayStatus::Paused => { /* TODO: reset then play */ }
                    },
                    StateChange::Bpm(_)
                    | StateChange::NextPage(_)
                    | StateChange::NextElement(_)
                    | StateChange::None
                    | StateChange::Sync(_) => {}
                }
                let _ = state_sender.send(state_change).await;
            }

            let tick_duration = seq::tick_duration(state.bpm.0 as f32);
            Timer::delay(tick_duration).await
        }
    }
}
