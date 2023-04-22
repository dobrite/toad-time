use core::fmt::Write;
use eg_pcf::{include_pcf, text::PcfTextStyle, PcfFont};
use heapless::String;

use rp_pico::hal::{
    gpio::pin::bank0::*,
    gpio::pin::PushPull,
    gpio::Output,
    gpio::Pin,
    pac,
    spi::{Enabled, Spi},
};

use embedded_graphics::{pixelcolor::BinaryColor, prelude::Point, text::Text, Drawable};

use ssd1306::Ssd1306;

pub type Ssd1306Display = Ssd1306<
    ssd1306::prelude::SPIInterface<
        Spi<Enabled, pac::SPI0, 8>,
        Pin<Gpio16, Output<PushPull>>,
        Pin<Gpio17, Output<PushPull>>,
    >,
    ssd1306::prelude::DisplaySize128x64,
    ssd1306::mode::BufferedGraphicsMode<ssd1306::prelude::DisplaySize128x64>,
>;

use crate::state::{State, StateChange};

const SMOL_FONT: PcfFont =
    include_pcf!("fonts/FrogPrincess-7.pcf", 'A'..='Z' | 'a'..='z' | '0'..='9' | ' ');
const BIGGE_FONT: PcfFont =
    include_pcf!("fonts/FrogPrincess-10.pcf", 'A'..='Z' | 'a'..='z' | '0'..='9' | ' ');

pub struct Display {
    display: Ssd1306Display,
    bigge_font: PcfTextStyle<'static, BinaryColor>,
    #[allow(dead_code)]
    smol_font: PcfTextStyle<'static, BinaryColor>,
    bpm_str: heapless::String<7>,
}

impl Display {
    pub fn new(initial_state: State, display: Ssd1306Display) -> Self {
        let bigge_font = PcfTextStyle::new(&BIGGE_FONT, BinaryColor::On);
        let smol_font = PcfTextStyle::new(&SMOL_FONT, BinaryColor::On);
        let bpm_str: String<7> = String::new();
        let mut display = Self {
            bigge_font,
            smol_font,
            bpm_str,
            display,
        };
        display.write_bpm(initial_state.bpm());
        display
    }

    pub fn handle_state_change(&mut self, state_change: StateChange) {
        match state_change {
            StateChange::Bpm(bpm) => self.write_bpm(bpm),
            StateChange::None => unreachable!(),
        }
    }

    fn write_bpm(&mut self, bpm: u32) {
        self.bpm_str.clear();
        self.display.clear();
        write!(self.bpm_str, "{} BPM", bpm).unwrap();

        Text::new(&self.bpm_str, Point::new(30, 70), self.bigge_font)
            .draw(&mut self.display)
            .unwrap();

        self.display.flush().unwrap();
    }
}
