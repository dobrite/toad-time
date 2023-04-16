#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

#[rtic::app(
    device = rp_pico::hal::pac,
    dispatchers = [TIMER_IRQ_1]
)]
mod app {
    use core::mem::MaybeUninit;
    use defmt::*;
    use defmt_rtt as _;
    use eg_pcf::{include_pcf, text::PcfTextStyle, PcfFont};
    use embedded_hal::{
        digital::v2::{InputPin, OutputPin, ToggleableOutputPin},
        spi,
    };
    use fugit::RateExtU32;
    use panic_probe as _;

    use rtic_monotonics::rp2040::{Timer, *};

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
    struct Shared {}

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

        tick::spawn().ok();
        display::spawn().ok();
        update_encoder::spawn().ok();
        update_encoder_button::spawn().ok();
        update_page_button::spawn().ok();
        update_play_button::spawn().ok();

        let mut led = pins.led.into_push_pull_output();
        led.set_high().unwrap();

        (
            Shared {},
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
    async fn update_play_button(ctx: update_play_button::Context) {
        debounced_button("play", ctx.local.play_button).await
    }

    #[task(local = [page_button], priority = 1)]
    async fn update_page_button(ctx: update_page_button::Context) {
        debounced_button("page", ctx.local.page_button).await
    }

    #[task(local = [encoder_button], priority = 1)]
    async fn update_encoder_button(ctx: update_encoder_button::Context) {
        debounced_button("encoder", ctx.local.encoder_button).await
    }

    async fn debounced_button<B: InputPin>(name: &str, button: &B)
    where
        <B as InputPin>::Error: core::fmt::Debug,
    {
        let mut armed = true;

        loop {
            if armed && button.is_low().unwrap() {
                armed = false;
                info!("{:?} button click!", name);
            } else if !armed && button.is_high().unwrap() {
                armed = true;
            }

            Timer::delay(BUTTON_UPDATE).await
        }
    }

    #[task(local = [encoder], priority = 1)]
    async fn update_encoder(ctx: update_encoder::Context) {
        let encoder = ctx.local.encoder;
        let update_duration = fugit::Duration::<u64, 1, MICRO_SECONDS>::from_ticks(1111);

        loop {
            encoder.update();
            match encoder.direction() {
                Direction::Clockwise => {
                    info!("clockwise")
                }
                Direction::Anticlockwise => {
                    info!("anticlockwise")
                }
                Direction::None => {}
            }
            Timer::delay(update_duration).await
        }
    }

    #[task(local = [display], priority = 1)]
    async fn display(ctx: display::Context) {
        let mut update = true;
        let bigge_font = PcfTextStyle::new(&BIGGE_FONT, BinaryColor::On);
        Text::new("BPM", Point::new(30, 50), bigge_font)
            .draw(*ctx.local.display)
            .unwrap();

        loop {
            if update {
                ctx.local.display.flush().unwrap();
                update = false
            }

            Timer::delay(10.millis()).await
        }
    }

    #[task(local = [led], priority = 1)]
    async fn tick(ctx: tick::Context) {
        loop {
            _ = ctx.local.led.toggle();

            Timer::delay(500.millis()).await
        }
    }
}
