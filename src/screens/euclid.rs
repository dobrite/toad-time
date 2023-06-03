use core::fmt::Write;

use embedded_graphics::prelude::Point;
use heapless::{String, Vec};
use seq::{Length, OutputConfig, OutputType, Rate};
use tinybmp::Bmp as TinyBmp;

use crate::{
    display::Bmp,
    screens::Display,
    state::{Output, OutputTypeString, RateString},
};

const GRID_START_X: usize = 54;
const GRID_START_Y: usize = 46;
const CLOCK: &[u8; 1318] = include_bytes!("../assets/icons/clock.bmp");
const CARET: &[u8; 78] = include_bytes!("../assets/icons/caret.bmp");
const STEP_ON: &[u8; 134] = include_bytes!("../assets/icons/step-on.bmp");
const STEP_OFF: &[u8; 134] = include_bytes!("../assets/icons/step-off.bmp");

pub struct EuclidScreen {
    caret: Bmp,
    clock: Bmp,
    name_str: String<3>,
    step_on: Bmp,
    step_off: Bmp,
}

impl EuclidScreen {
    pub fn new() -> Self {
        let caret = TinyBmp::from_slice(CARET).unwrap();
        let clock = TinyBmp::from_slice(CLOCK).unwrap();
        let step_on = TinyBmp::from_slice(STEP_ON).unwrap();
        let step_off = TinyBmp::from_slice(STEP_OFF).unwrap();

        Self {
            caret,
            clock,
            name_str: String::new(),
            step_on,
            step_off,
        }
    }

    pub fn draw(&mut self, name: &Output, config: &OutputConfig, display: &mut Display) {
        self.draw_name(name, display);
        self.draw_clock(display);
        self.draw_rate(&config.rate(), display);
        self.draw_length(&config.length(), display);
        self.draw_grid(config.index(), config.sequence(), display);
        self.draw_output_type(&config.output_type(), display);
    }

    fn draw_name(&mut self, output: &Output, display: &mut Display) {
        self.name_str.clear();
        write!(self.name_str, "{}", output).unwrap();

        display.draw_bigge_text(&self.name_str, Point::new(0, 24));
    }

    fn draw_clock(&mut self, display: &mut Display) {
        display.draw_bmp(&self.clock, Point::new(54, 8));
    }

    fn draw_rate(&mut self, rate: &Rate, display: &mut Display) {
        display.draw_smol_text(&RateString::from(rate).0, Point::new(72, 29));
    }

    fn draw_length(&mut self, length: &Length, display: &mut Display) {
        let s: String<3> = String::from(length.0);
        display.draw_smol_text(&s, Point::new(74, 45));
    }

    fn draw_grid(&mut self, index: usize, sequence: &Vec<bool, 16>, display: &mut Display) {
        let len = sequence.len();
        for idx in 0..len {
            let x = idx % len % 8;
            let y = idx / 8;
            let step_on = sequence[idx];
            let step_bmp = if step_on { self.step_on } else { self.step_off };
            let p_x = (GRID_START_X + x * 7) as i32;
            let p_y = (GRID_START_Y + y * 10) as i32;
            if idx == index {
                display.draw_bmp(&self.caret, Point::new(p_x + 1, p_y - 3));
            }

            display.draw_bmp(&step_bmp, Point::new(p_x, p_y));
        }
    }

    fn draw_output_type(&mut self, output_type: &OutputType, display: &mut Display) {
        display.draw_bigge_text(&OutputTypeString::from(output_type).0, Point::new(0, 50));
    }
}
