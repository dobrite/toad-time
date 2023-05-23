use core::fmt::Write;

use embedded_graphics::prelude::Point;
use heapless::String;
use seq::{Density, Length, OutputConfig, Rate};
use tinybmp::Bmp as TinyBmp;

use crate::{
    display::Bmp,
    screens::Display,
    state::{Output, RateString},
};

const CLOCK: &[u8; 1318] = include_bytes!("../assets/icons/Clock.bmp");

pub struct EuclidScreen {
    clock: Bmp,
    name_str: String<3>,
}

impl EuclidScreen {
    pub fn new() -> Self {
        let clock = TinyBmp::from_slice(CLOCK).unwrap();

        Self {
            clock,
            name_str: String::new(),
        }
    }

    pub fn draw(&mut self, name: &Output, config: &OutputConfig, display: &mut Display) {
        self.draw_name(name, display);
        self.draw_clock(display);
        self.draw_rate(config.rate, display);
        self.draw_length(config.length, display);
        self.draw_density(config.density, display);
    }

    fn draw_name(&mut self, output: &Output, display: &mut Display) {
        self.name_str.clear();
        write!(self.name_str, "{}", output).unwrap();

        display.draw_bigge_text(&self.name_str, Point::new(0, 24));
    }

    fn draw_clock(&mut self, display: &mut Display) {
        display.draw_bmp(&self.clock, Point::new(54, 8));
    }

    fn draw_rate(&mut self, rate: Rate, display: &mut Display) {
        display.draw_smol_text(&RateString::from(rate).0, Point::new(72, 29));
    }

    fn draw_length(&mut self, length: Length, display: &mut Display) {
        let s: String<3> = String::from(length.0);
        display.draw_smol_text(&s, Point::new(74, 45));
    }

    fn draw_density(&mut self, density: Density, display: &mut Display) {
        let s: String<3> = String::from(density.0);
        display.draw_smol_text(&s, Point::new(74, 60));
    }
}
