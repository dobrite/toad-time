#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

mod state;

#[rtic::app(
    device = rp_pico::hal::pac,
    dispatchers = [TIMER_IRQ_1, TIMER_IRQ_2]
)]
mod app {
    use crate::state::{Command, State, StateChange, COMMAND_CAPACITY, STATE_CHANGE_CAPACITY};
    use core::{fmt::Write, mem::MaybeUninit};
    use defmt::info;
    use defmt_rtt as _;
    use eg_pcf::{include_pcf, text::PcfTextStyle, PcfFont};
    use embedded_hal::{
        digital::v2::{InputPin, OutputPin, ToggleableOutputPin},
        spi,
    };
    use fugit::RateExtU32;
    use heapless::String;
    use panic_probe as _;

    use rtic_monotonics::rp2040::{Timer, *};
    use rtic_sync::{channel::*, make_channel};

    use rp_pico::{
        hal::{
            self, clocks, gpio,
            gpio::pin::bank0::*,
            gpio::pin::{PullUp, PushPull, PushPullOutput},
            gpio::{Input, Output},
            pac,
            sio::Sio,
            spi::{Enabled, Spi},
            watchdog::Watchdog,
            Clock,
        },
        XOSC_CRYSTAL_FREQ,
    };

    use embedded_graphics::{pixelcolor::BinaryColor, prelude::*, text::Text};
    use ssd1306::{prelude::*, Ssd1306};

    use rotary_encoder_embedded::{standard::StandardMode, Direction, RotaryEncoder};

    type Display = Ssd1306<
        ssd1306::prelude::SPIInterface<
            Spi<Enabled, pac::SPI0, 8>,
            gpio::Pin<Gpio16, Output<PushPull>>,
            gpio::Pin<Gpio17, Output<PushPull>>,
        >,
        ssd1306::prelude::DisplaySize128x64,
        ssd1306::mode::BufferedGraphicsMode<ssd1306::prelude::DisplaySize128x64>,
    >;

    type Encoder = RotaryEncoder<
        StandardMode,
        gpio::Pin<Gpio14, Input<PullUp>>,
        gpio::Pin<Gpio15, Input<PullUp>>,
    >;

    // const SMOL_FONT: PcfFont = include_pcf!("fonts/FrogPrincess-7.pcf", 'A'..='Z' | 'a'..='z' | '0'..='9' | ' ');
    const BIGGE_FONT: PcfFont =
        include_pcf!("fonts/FrogPrincess-10.pcf", 'A'..='Z' | 'a'..='z' | '0'..='9' | ' ');
    const FIFTY_MILLI_SECONDS: u64 = 50_000;
    const MICRO_SECONDS: u32 = 1_000_000;
    const BUTTON_UPDATE: fugit::Duration<u64, 1, MICRO_SECONDS> =
        fugit::Duration::<u64, 1, MICRO_SECONDS>::from_ticks(FIFTY_MILLI_SECONDS);

    #[shared]
    struct Shared {
        state: State,
    }

    #[local]
    struct Local {
        display: &'static mut Display,
        encoder: Encoder,
        encoder_button: gpio::Pin<Gpio13, Input<PullUp>>,
        led: gpio::Pin<Gpio25, PushPullOutput>,
        play_button: gpio::Pin<Gpio11, Input<PullUp>>,
        page_button: gpio::Pin<Gpio12, Input<PullUp>>,
    }

    #[init(local=[display_ctx: MaybeUninit<Display> = MaybeUninit::uninit()])]
    fn init(mut ctx: init::Context) -> (Shared, Local) {
        unsafe {
            hal::sio::spinlock_reset();
        }

        info!("program start");

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

        let oled_dc = pins.gpio16.into_push_pull_output();
        let oled_cs = pins.gpio17.into_push_pull_output();
        let _ = pins
            .gpio18
            .into_mode::<rp_pico::hal::gpio::pin::FunctionSpi>();
        let _ = pins
            .gpio19
            .into_mode::<rp_pico::hal::gpio::pin::FunctionSpi>();
        let mut oled_reset = pins.gpio20.into_push_pull_output();

        let spi = Spi::<_, _, 8>::new(ctx.device.SPI0).init(
            &mut ctx.device.RESETS,
            125_000_000u32.Hz(),
            1_000_000u32.Hz(),
            &spi::MODE_0,
        );

        let interface = SPIInterface::new(spi, oled_dc, oled_cs);
        let display_ctx: &'static mut _ = ctx.local.display_ctx.write(
            Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
                .into_buffered_graphics_mode(),
        );

        display_ctx.reset(&mut oled_reset, &mut delay).unwrap();
        display_ctx.init().unwrap();

        let play_button = pins.gpio11.into_pull_up_input();
        let page_button = pins.gpio12.into_pull_up_input();
        let encoder_button = pins.gpio13.into_pull_up_input();
        let rotary_dt = pins.gpio14.into_pull_up_input();
        let rotary_clk = pins.gpio15.into_pull_up_input();
        let encoder = RotaryEncoder::new(rotary_dt, rotary_clk).into_standard_mode();

        let (command_sender, command_receiver) = make_channel!(Command, COMMAND_CAPACITY);
        let (state_sender, state_receiver) = make_channel!(StateChange, STATE_CHANGE_CAPACITY);

        let initial_state: State = Default::default();

        tick::spawn().ok();
        state::spawn(command_receiver, state_sender).ok();
        display::spawn(initial_state, state_receiver).ok();
        update_encoder::spawn(command_sender.clone()).ok();
        update_encoder_button::spawn(command_sender.clone()).ok();
        update_page_button::spawn(command_sender.clone()).ok();
        update_play_button::spawn(command_sender).ok();

        let mut led = pins.led.into_push_pull_output();
        led.set_high().unwrap();

        (
            Shared {
                state: State { bpm: 120 },
            },
            Local {
                display: display_ctx,
                encoder,
                encoder_button,
                led,
                page_button,
                play_button,
            },
        )
    }

    #[idle(local = [])]
    fn idle(_cx: idle::Context) -> ! {
        info!("idle!");

        loop {
            cortex_m::asm::nop();
        }
    }

    #[task(local = [play_button], priority = 1)]
    async fn update_play_button(
        ctx: update_play_button::Context,
        sender: Sender<'static, Command, COMMAND_CAPACITY>,
    ) {
        debounced_button("play", sender, ctx.local.play_button, Command::PlayPress).await
    }

    #[task(local = [page_button], priority = 1)]
    async fn update_page_button(
        ctx: update_page_button::Context,

        sender: Sender<'static, Command, COMMAND_CAPACITY>,
    ) {
        debounced_button("page", sender, ctx.local.page_button, Command::PagePress).await
    }

    #[task(local = [encoder_button], priority = 1)]
    async fn update_encoder_button(
        ctx: update_encoder_button::Context,
        sender: Sender<'static, Command, COMMAND_CAPACITY>,
    ) {
        debounced_button(
            "encoder",
            sender,
            ctx.local.encoder_button,
            Command::EncoderPress,
        )
        .await
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

    async fn debounced_button<B: InputPin>(
        name: &str,
        mut sender: Sender<'static, Command, COMMAND_CAPACITY>,
        button: &B,
        command: Command,
    ) where
        <B as InputPin>::Error: core::fmt::Debug,
    {
        let mut armed = true;

        loop {
            if armed && button.is_low().unwrap() {
                armed = false;
                info!("{:?} button click!", name);
                let _ = sender.send(command).await;
            } else if !armed && button.is_high().unwrap() {
                armed = true;
            }

            Timer::delay(BUTTON_UPDATE).await
        }
    }

    #[task(local = [encoder], priority = 1)]
    async fn update_encoder(
        ctx: update_encoder::Context,
        mut sender: Sender<'static, Command, COMMAND_CAPACITY>,
    ) {
        let encoder = ctx.local.encoder;
        let update_duration = fugit::Duration::<u64, 1, MICRO_SECONDS>::from_ticks(1111);

        loop {
            encoder.update();
            match encoder.direction() {
                Direction::Clockwise => {
                    info!("clockwise");
                    let _ = sender.send(Command::EncoderRight).await;
                }
                Direction::Anticlockwise => {
                    info!("anticlockwise");
                    let _ = sender.send(Command::EncoderLeft).await;
                }
                Direction::None => {}
            }
            Timer::delay(update_duration).await
        }
    }

    #[task(local = [display], priority = 1)]
    async fn display(
        ctx: display::Context,
        initial_state: State,
        mut receiver: Receiver<'static, StateChange, 4>,
    ) {
        let display = ctx.local.display;
        let bigge_font = PcfTextStyle::new(&BIGGE_FONT, BinaryColor::On);
        let mut bpm_str: String<7> = String::new();
        write!(bpm_str, "{} BPM", initial_state.bpm).unwrap();

        Text::new(&bpm_str, Point::new(30, 70), bigge_font)
            .draw(*display)
            .unwrap();

        display.flush().unwrap();

        while let Ok(state_change) = receiver.recv().await {
            match state_change {
                StateChange::Bpm(bpm) => {
                    bpm_str.clear();
                    display.clear();
                    write!(bpm_str, "{} BPM", bpm).unwrap();
                    info!("{:?}", bpm);

                    Text::new(&bpm_str, Point::new(30, 70), bigge_font)
                        .draw(*display)
                        .unwrap();
                }
                StateChange::None => unreachable!(),
            }

            display.flush().unwrap();
        }
    }

    const MAX_MULT: u8 = 192;
    const PWM_PERCENT_INCREMENTS: u8 = 10;
    const SECONDS_IN_MINUTES: u8 = 60;

    #[task(local = [led], shared = [state], priority = 2)]
    async fn tick(mut ctx: tick::Context) {
        let milli_seconds_per_tick = ctx.shared.state.lock(|state| {
            state.bpm as f32
                / SECONDS_IN_MINUTES as f32
                / PWM_PERCENT_INCREMENTS as f32
                / MAX_MULT as f32
                * MICRO_SECONDS as f32
        });
        info!("us per tick: {:?}", milli_seconds_per_tick); // 1.0416667
        let tick_duration =
            fugit::Duration::<u64, 1, MICRO_SECONDS>::from_ticks(milli_seconds_per_tick as u64);

        let target = 240; // 0.25 seconds == 120 bpm at 50% PWM
        let mut counter = 0;

        loop {
            if counter == target {
                _ = ctx.local.led.toggle();
                counter = 0;
            } else {
                counter += 1;
            }

            Timer::delay(tick_duration).await
        }
    }
}
