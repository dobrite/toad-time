use core::fmt::Write;

use embedded_graphics::prelude::Point;
use heapless::String;

use crate::screens::{Display, ScreenState};

pub struct Gate {
    name: String<3>,
}

impl Gate {
    pub fn new(name: &str) -> Self {
        let mut name_str = String::new();
        write!(name_str, "{}", name).unwrap();

        Self { name: name_str }
    }

    pub fn draw(&mut self, _state: &ScreenState, display: &mut Display) {
        self.draw_name(display);
    }

    fn draw_name(&mut self, display: &mut Display) {
        display.draw_bigge_text(&self.name, Point::new(0, 24));
    }
}
