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
        digital::v2::{OutputPin, ToggleableOutputPin},
        spi,
    };
    use fugit::RateExtU32;
    use panic_probe as _;

    use rtic_monotonics::rp2040::{Timer, *};

    use rp_pico::{
        hal::{
            self, clocks, gpio,
            gpio::pin::bank0::{Gpio16, Gpio17, Gpio25},
            gpio::pin::{PushPull, PushPullOutput},
            gpio::Output,
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

    type Display = Ssd1306<
        ssd1306::prelude::SPIInterface<
            Spi<Enabled, pac::SPI0, 8>,
            gpio::Pin<Gpio16, Output<PushPull>>,
            gpio::Pin<Gpio17, Output<PushPull>>,
        >,
        ssd1306::prelude::DisplaySize128x64,
        ssd1306::mode::BufferedGraphicsMode<ssd1306::prelude::DisplaySize128x64>,
    >;

    // const SMOL_FONT: PcfFont = include_pcf!("fonts/FrogPrincess-7.pcf", 'A'..='Z' | 'a'..='z' | '0'..='9' | ' ');
    const BIGGE_FONT: PcfFont =
        include_pcf!("fonts/FrogPrincess-10.pcf", 'A'..='Z' | 'a'..='z' | '0'..='9' | ' ');

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: gpio::Pin<Gpio25, PushPullOutput>,
        display: &'static mut Display,
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

        heartbeat::spawn().ok();

        let mut led = pins.led.into_push_pull_output();
        info!("led high!");
        led.set_high().unwrap();

        (
            Shared {},
            Local {
                led,
                display: display_ctx,
            },
        )
    }

    //#[idle(local = [])]
    //fn idle(_cx: idle::Context) -> ! {
    //    info!("idle!");

    //    loop {
    //        cortex_m::asm::nop();
    //    }
    //}

    #[task(local = [led, display, times: u32 = 0], priority = 1)]
    async fn heartbeat(ctx: heartbeat::Context) {
        loop {
            info!("task!");
            // Flicker the built-in LED
            _ = ctx.local.led.toggle();

            let bigge_font = PcfTextStyle::new(&BIGGE_FONT, BinaryColor::On);
            Text::new("BPM", Point::new(30, 50), bigge_font)
                .draw(*ctx.local.display)
                .unwrap();

            *ctx.local.times += 1;

            ctx.local.display.flush().unwrap();
            // Congrats, you can use your i2c and have access to it here,
            // now to do something with it!
            info!("after toggle!");

            // Re-spawn this task after 1 second
            Timer::delay(500.millis()).await
        }
    }
}
