#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

mod display;
mod screens;
mod state;

#[rtic::app(
    device = rp_pico::hal::pac,
    dispatchers = [TIMER_IRQ_1, TIMER_IRQ_2]
)]
mod app {
    use defmt_rtt as _;
    use embedded_hal::{
        digital::v2::{InputPin, OutputPin},
        spi,
    };
    use fugit::RateExtU32;
    use panic_probe as _;
    use rotary_encoder_embedded::{standard::StandardMode, Direction, RotaryEncoder};
    use rp_pico::{
        hal::{
            self, clocks,
            gpio::{
                pin::{bank0::*, PullUp, PushPullOutput},
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
    use seq::Outputs;
    use ssd1306::{prelude::*, Ssd1306};

    use super::{
        display::Display,
        screens::Screens,
        state::{
            Command, MicroSeconds, State, StateChange, COMMAND_CAPACITY, MAX_MULT,
            PWM_PERCENT_INCREMENTS, STATE_CHANGE_CAPACITY,
        },
    };

    type Encoder =
        RotaryEncoder<StandardMode, Pin<Gpio14, Input<PullUp>>, Pin<Gpio15, Input<PullUp>>>;

    #[shared]
    struct Shared {
        state: State,
    }

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
            let _ = pins
                .gpio18
                .into_mode::<rp_pico::hal::gpio::pin::FunctionSpi>();
            let _ = pins
                .gpio19
                .into_mode::<rp_pico::hal::gpio::pin::FunctionSpi>();
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
        let (state_sender, state_receiver) = make_channel!(StateChange, STATE_CHANGE_CAPACITY);

        tick::spawn().ok();
        state::spawn(command_receiver, state_sender).ok();
        display::spawn(state_receiver).ok();
        update_encoder::spawn(command_sender.clone()).ok();
        update_encoder_button::spawn(command_sender.clone()).ok();
        update_page_button::spawn(command_sender.clone()).ok();
        update_play_button::spawn(command_sender).ok();

        let gate_a = pins.gpio2.into_push_pull_output();
        let gate_b = pins.gpio3.into_push_pull_output();
        let gate_c = pins.gpio4.into_push_pull_output();
        let gate_d = pins.gpio5.into_push_pull_output();

        (
            Shared {
                state: State::new(),
            },
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
        sender: Sender<'static, Command, COMMAND_CAPACITY>,
    ) {
        debounced_button(sender, ctx.local.play_button, Command::PlayPress).await
    }

    #[task(local = [page_button], priority = 1)]
    async fn update_page_button(
        ctx: update_page_button::Context,

        sender: Sender<'static, Command, COMMAND_CAPACITY>,
    ) {
        debounced_button(sender, ctx.local.page_button, Command::PagePress).await
    }

    #[task(local = [encoder_button], priority = 1)]
    async fn update_encoder_button(
        ctx: update_encoder_button::Context,
        sender: Sender<'static, Command, COMMAND_CAPACITY>,
    ) {
        debounced_button(sender, ctx.local.encoder_button, Command::EncoderPress).await
    }

    async fn debounced_button<B: InputPin>(
        mut sender: Sender<'static, Command, COMMAND_CAPACITY>,
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
                let _ = sender.send(command).await;
            } else if !armed && button.is_high().unwrap() {
                armed = true;
            }

            Timer::delay(button_update_duration).await
        }
    }

    #[task(local = [encoder], priority = 1)]
    async fn update_encoder(
        ctx: update_encoder::Context,
        mut sender: Sender<'static, Command, COMMAND_CAPACITY>,
    ) {
        let encoder = ctx.local.encoder;
        let update_duration = MicroSeconds::from_ticks(1111);

        loop {
            encoder.update();
            match encoder.direction() {
                Direction::Clockwise => {
                    let _ = sender.send(Command::EncoderRight).await;
                }
                Direction::Anticlockwise => {
                    let _ = sender.send(Command::EncoderLeft).await;
                }
                Direction::None => {}
            }
            Timer::delay(update_duration).await
        }
    }

    #[task(local = [], shared = [state], priority = 1)]
    async fn state(
        mut ctx: state::Context,
        mut receiver: Receiver<'static, Command, COMMAND_CAPACITY>,
        mut sender: Sender<'static, StateChange, STATE_CHANGE_CAPACITY>,
    ) {
        while let Ok(command) = receiver.recv().await {
            let state_change = ctx.shared.state.lock(|state| state.handle_command(command));
            match state_change {
                StateChange::None => {}
                _ => {
                    let _ = sender.send(state_change).await;
                }
            }
        }
    }

    #[task(local = [display], priority = 1)]
    async fn display(ctx: display::Context, mut receiver: Receiver<'static, StateChange, 4>) {
        let display = ctx.local.display;
        let mut screens = Screens::new();
        screens.handle_state_change(StateChange::Initialize, display);

        while let Ok(state_change) = receiver.recv().await {
            screens.handle_state_change(state_change, display);
        }
    }

    #[task(local = [gate_a, gate_b, gate_c, gate_d], shared = [state], priority = 2)]
    async fn tick(mut ctx: tick::Context) {
        let resolution = PWM_PERCENT_INCREMENTS * MAX_MULT;
        let mut outputs = Outputs::new(4, resolution);

        let mut tick_duration = ctx.shared.state.lock(|state| {
            let tick_duration = state.tick_duration();
            state
                .gate_configs()
                .iter()
                .enumerate()
                .for_each(|(index, config)| {
                    outputs.set_pwm(index, config.pwm);
                    outputs.set_rate(index, config.rate);
                    outputs.set_prob(index, config.prob);
                });

            tick_duration
        });

        loop {
            let tick = outputs.tick();
            let result = outputs.state();

            tick_duration = ctx.shared.state.lock(|state| {
                tick_duration = state.tick_duration();
                if tick.major {
                    state
                        .gate_configs()
                        .iter()
                        .enumerate()
                        .for_each(|(index, config)| {
                            outputs.set_pwm(index, config.pwm);
                            outputs.set_rate(index, config.rate);
                            outputs.set_prob(index, config.prob);
                        });
                }

                tick_duration
            });

            if result.outputs[0] {
                _ = ctx.local.gate_a.set_high();
            } else {
                _ = ctx.local.gate_a.set_low();
            }

            if result.outputs[1] {
                _ = ctx.local.gate_b.set_high();
            } else {
                _ = ctx.local.gate_b.set_low();
            }

            if result.outputs[2] {
                _ = ctx.local.gate_c.set_high();
            } else {
                _ = ctx.local.gate_c.set_low();
            }

            if result.outputs[3] {
                _ = ctx.local.gate_d.set_high();
            } else {
                _ = ctx.local.gate_d.set_low();
            }

            Timer::delay(tick_duration).await
        }
    }
}
