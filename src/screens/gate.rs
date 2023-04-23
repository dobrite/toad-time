use core::fmt::Write;

use embedded_graphics::{
    prelude::{Point, Size},
    primitives::Rectangle,
};
use heapless::String;
use tinybmp::Bmp as TinyBmp;

use crate::{
    display::Bmp,
    screens::{Display, ScreenState, POINTER},
    state,
    state::{Element, GateElement},
};

const CLOCK: &[u8; 1318] = include_bytes!("../assets/icons/Clock.bmp");
const PWM: &[u8; 2166] = include_bytes!("../assets/icons/PWMSpritesheetSmol.bmp");

pub struct Gate {
    clock: Bmp,
    name: String<3>,
    pointer: Bmp,
    pwm: Bmp,
}

impl Gate {
    pub fn new(gate: state::Gate) -> Self {
        let mut name = String::new();
        write!(name, "{}", gate).unwrap();

        let clock = TinyBmp::from_slice(CLOCK).unwrap();
        let pointer = TinyBmp::from_slice(POINTER).unwrap();
        let pwm = TinyBmp::from_slice(PWM).unwrap();

        Self {
            clock,
            name,
            pointer,
            pwm,
        }
    }

    pub fn draw(&mut self, state: &ScreenState, display: &mut Display) {
        self.draw_name(display);
        self.draw_pointer(display, state.current);
        self.draw_clock(display);
        self.draw_div(display);
        self.draw_pwm(display); // 65x16 (13x8)
    }

    fn draw_name(&mut self, display: &mut Display) {
        display.draw_bigge_text(&self.name, Point::new(0, 24));
    }

    fn draw_pointer(&mut self, display: &mut Display, current: Element) {
        let point = match current {
            Element::Gate(_, GateElement::Div) => Point::new(36, 10),
            Element::Gate(_, GateElement::Pwm) => Point::new(36, 28),
            Element::Home(..) => unreachable!(),
        };
        display.draw_bmp(&self.pointer, point);
    }

    fn draw_clock(&mut self, display: &mut Display) {
        display.draw_bmp(&self.clock, Point::new(54, 8));
    }

    fn draw_div(&mut self, display: &mut Display) {
        display.draw_smol_text(&"x1".into(), Point::new(72, 29));
    }

    fn draw_pwm(&mut self, display: &mut Display) {
        let rectangle = Rectangle::new(Point::new(52, 0), Size::new(65, 8));
        display.draw_sub_bmp(&self.pwm, &rectangle, Point::new(70, 30));
    }
}
