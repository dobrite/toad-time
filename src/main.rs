#![no_std]
#![no_main]

use bsp::entry;
use defmt::*;
use defmt_rtt as _;
use eg_pcf::{include_pcf, text::PcfTextStyle, PcfFont};
use embedded_hal::{digital::v2::OutputPin, spi};
use fugit::RateExtU32;
use panic_probe as _;
use rp_pico as bsp;

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    spi::Spi,
    watchdog::Watchdog,
};

use embedded_graphics::{pixelcolor::BinaryColor, prelude::*, text::Text};
use ssd1306::{prelude::*, Ssd1306};

// const SMOL_FONT: PcfFont = include_pcf!("fonts/FrogPrincess-7.pcf", 'A'..='Z' | 'a'..='z' | '0'..='9' | ' ');
const BIGGE_FONT: PcfFont =
    include_pcf!("fonts/FrogPrincess-10.pcf", 'A'..='Z' | 'a'..='z' | '0'..='9' | ' ');

#[entry]
fn main() -> ! {
    info!("program start");
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let oled_dc = pins.gpio16.into_push_pull_output();
    let oled_cs = pins.gpio17.into_push_pull_output();
    let _ = pins.gpio18.into_mode::<bsp::hal::gpio::pin::FunctionSpi>();
    let _ = pins.gpio19.into_mode::<bsp::hal::gpio::pin::FunctionSpi>();
    let mut oled_reset = pins.gpio20.into_push_pull_output();

    let spi = Spi::<_, _, 8>::new(pac.SPI0).init(
        &mut pac.RESETS,
        125_000_000u32.Hz(),
        1_000_000u32.Hz(),
        &spi::MODE_0,
    );

    let interface = SPIInterface::new(spi, oled_dc, oled_cs);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    display.reset(&mut oled_reset, &mut delay).unwrap();
    display.init().unwrap();

    let bigge_font = PcfTextStyle::new(&BIGGE_FONT, BinaryColor::On);
    Text::new("BPM", Point::new(30, 50), bigge_font)
        .draw(&mut display)
        .unwrap();

    display.flush().unwrap();

    let mut led_pin = pins.led.into_push_pull_output();

    loop {
        info!("on!");
        led_pin.set_high().unwrap();
        delay.delay_ms(5000);
        info!("off!");
        led_pin.set_low().unwrap();
        delay.delay_ms(5000);
    }
}
