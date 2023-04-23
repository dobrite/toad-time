use core::fmt::Write;

use embedded_graphics::prelude::Point;
use heapless::String;

use crate::{
    screens::{Display, ScreenState},
    state,
};

pub struct Gate {
    name: String<3>,
}

impl Gate {
    pub fn new(gate: state::Gate) -> Self {
        let mut name = String::new();
        write!(name, "{}", gate).unwrap();

        Self { name }
    }

    pub fn draw(&mut self, _state: &ScreenState, display: &mut Display) {
        self.draw_name(display);
    }

    fn draw_name(&mut self, display: &mut Display) {
        display.draw_bigge_text(&self.name, Point::new(0, 24));
    }
}
