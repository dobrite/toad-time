#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

#[rtic::app(
    device = rp_pico::hal::pac,
    dispatchers = [TIMER_IRQ_1]
)]
mod app {
    use defmt::*;
    use defmt_rtt as _;
    use eg_pcf::{include_pcf, text::PcfTextStyle, PcfFont};
    use embedded_hal::{
        digital::v2::{OutputPin, ToggleableOutputPin},
        spi,
    };
    use fugit::RateExtU32;
    use panic_probe as _;

    use rtic_monotonics::rp2040::{Timer, *};

    use rp_pico::{
        hal::{
            clocks, gpio,
            gpio::pin::bank0::{Gpio2, Gpio25, Gpio3},
            gpio::pin::PushPullOutput,
            pac,
            sio::Sio,
            spi::Spi,
            watchdog::Watchdog,
            Clock, I2C,
        },
        XOSC_CRYSTAL_FREQ,
    };

    use embedded_graphics::{pixelcolor::BinaryColor, prelude::*, text::Text};
    use ssd1306::{prelude::*, Ssd1306};

    // const SMOL_FONT: PcfFont = include_pcf!("fonts/FrogPrincess-7.pcf", 'A'..='Z' | 'a'..='z' | '0'..='9' | ' ');
    //const BIGGE_FONT: PcfFont =
    //    include_pcf!("fonts/FrogPrincess-10.pcf", 'A'..='Z' | 'a'..='z' | '0'..='9' | ' ');

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: gpio::Pin<Gpio25, PushPullOutput>,
    }

    #[init()]
    fn init(mut ctx: init::Context) -> (Shared, Local) {
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

        //let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

        let sio = Sio::new(ctx.device.SIO);
        let pins = rp_pico::Pins::new(
            ctx.device.IO_BANK0,
            ctx.device.PADS_BANK0,
            sio.gpio_bank0,
            &mut ctx.device.RESETS,
        );

        //let oled_dc = pins.gpio16.into_push_pull_output();
        //let oled_cs = pins.gpio17.into_push_pull_output();
        //let _ = pins
        //    .gpio18
        //    .into_mode::<rp_pico::hal::gpio::pin::FunctionSpi>();
        //let _ = pins
        //    .gpio19
        //    .into_mode::<rp_pico::hal::gpio::pin::FunctionSpi>();
        //let mut oled_reset = pins.gpio20.into_push_pull_output();

        //let spi = Spi::<_, _, 8>::new(pac.SPI0).init(
        //    &mut pac.RESETS,
        //    125_000_000u32.Hz(),
        //    1_000_000u32.Hz(),
        //    &spi::MODE_0,
        //);

        //let interface = SPIInterface::new(spi, oled_dc, oled_cs);
        //let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        //    .into_buffered_graphics_mode();

        //display.reset(&mut oled_reset, &mut delay).unwrap();
        //display.init().unwrap();

        //let bigge_font = PcfTextStyle::new(&BIGGE_FONT, BinaryColor::On);
        //Text::new("BPM", Point::new(30, 50), bigge_font)
        //    .draw(&mut display)
        //    .unwrap();

        //display.flush().unwrap();

        heartbeat::spawn().ok();

        let mut led = pins.led.into_push_pull_output();
        info!("led high!");
        led.set_high().unwrap();

        (Shared {}, Local { led })
    }

    //#[idle(local = [])]
    //fn idle(_cx: idle::Context) -> ! {
    //    info!("idle!");

    //    loop {
    //        cortex_m::asm::nop();
    //    }
    //}

    #[task(local = [led], priority = 1)]
    async fn heartbeat(ctx: heartbeat::Context) {
        info!("task!");
        // Flicker the built-in LED
        _ = ctx.local.led.toggle();

        // Congrats, you can use your i2c and have access to it here,
        // now to do something with it!
        info!("after toggle!");

        // Re-spawn this task after 1 second
        Timer::delay(1000.millis()).await;
    }
}
