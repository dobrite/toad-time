use core::fmt::Write;

use embedded_graphics::{
    prelude::{Point, Size},
    primitives::Rectangle,
};
use heapless::String;
use tinybmp::Bmp as TinyBmp;

use crate::{
    display::Bmp,
    screens::Display,
    state::{Gate, GateState},
};

const CLOCK: &[u8; 1318] = include_bytes!("../assets/icons/Clock.bmp");
const PWM: &[u8; 2166] = include_bytes!("../assets/icons/PWMSpritesheetSmol.bmp");

pub struct GateScreen {
    clock: Bmp,
    gate: Gate,
    name: String<3>,
    pwm: Bmp,
}

impl GateScreen {
    pub fn new(gate: Gate) -> Self {
        let mut name = String::new();
        write!(name, "{}", gate).unwrap();

        let clock = TinyBmp::from_slice(CLOCK).unwrap();
        let pwm = TinyBmp::from_slice(PWM).unwrap();

        Self {
            clock,
            gate,
            name,
            pwm,
        }
    }

    pub fn draw(&mut self, state: &GateState, display: &mut Display) {
        self.draw_name(display);
        self.draw_clock(display);
        self.draw_rate(display);
        self.draw_pwm(display); // 65x16 (13x8)
    }

    fn draw_name(&mut self, display: &mut Display) {
        display.draw_bigge_text(&self.name, Point::new(0, 24));
    }

    fn draw_clock(&mut self, display: &mut Display) {
        display.draw_bmp(&self.clock, Point::new(54, 8));
    }

    fn draw_rate(&mut self, display: &mut Display) {
        display.draw_smol_text(&"x1".into(), Point::new(72, 29));
    }

    fn draw_pwm(&mut self, display: &mut Display) {
        let rectangle = Rectangle::new(Point::new(52, 0), Size::new(65, 8));
        display.draw_sub_bmp(&self.pwm, &rectangle, Point::new(70, 30));
    }
}
