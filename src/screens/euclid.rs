use embedded_graphics::prelude::Point;
use heapless::{String, Vec};
use seq::{Length, OutputConfig, OutputType, Rate};

use crate::{
    screens::Display,
    state::{Element, Output, OutputTypeString, RateString, Screen},
    StateChange,
};

const GRID_START_X: usize = 54;
const GRID_START_Y: usize = 46;

pub struct EuclidScreen {
    length_str: String<3>,
    name_str: String<3>,
    output_type_str: String<3>,
    rate_str: String<3>,
}

impl EuclidScreen {
    pub fn new() -> Self {
        Self {
            length_str: String::new(),
            name_str: String::new(),
            output_type_str: String::new(),
            rate_str: String::new(),
        }
    }

    pub fn draw(
        &mut self,
        state_change: &StateChange,
        config: &OutputConfig,
        display: &mut Display,
    ) {
        match state_change {
            StateChange::Rate(_, rate) => {
                self.clear_rate(display);
                self.draw_rate(display, rate);
            }
            StateChange::Length(_, length) => {
                self.clear_length(display);
                self.draw_length(display, length);
                self.clear_grid(display);
                self.draw_grid(display, config.sequence());
            }
            StateChange::OutputType(output, _) => {
                display.clear();
                self.draw_screen(display, output, config);
                self.draw_pointer(display, &Element::OutputType);
            }
            StateChange::Density(_, _) => {
                self.clear_grid(display);
                self.draw_grid(display, config.sequence());
            }
            StateChange::Index(..) => {
                self.draw_caret(display, config.index(), config.sequence().len());
            }
            StateChange::NextElement(element) => {
                self.draw_pointer(display, element);
            }
            StateChange::NextScreen(Screen::Output(output, _)) => {
                display.clear();
                self.draw_screen(display, output, config);
                self.draw_pointer(display, &Element::Rate);
            }
            _ => {}
        }
    }

    fn draw_screen(&mut self, display: &mut Display, output: &Output, config: &OutputConfig) {
        self.draw_name(display, output);
        self.draw_clock(display);
        self.draw_rate(display, &config.rate());
        self.draw_length(display, &config.length());
        self.draw_grid(display, config.sequence());
        self.draw_caret(display, config.index(), config.sequence().len());
        self.draw_output_type(display, &config.output_type());
    }

    fn draw_name(&mut self, display: &mut Display, output: &Output) {
        display.draw_bigge_text(&mut self.name_str, output, Point::new(0, 24));
    }

    fn draw_clock(&mut self, display: &mut Display) {
        display.draw_clock(Point::new(54, 8));
    }

    fn clear_rate(&mut self, display: &mut Display) {
        display.clear_smol_text(&self.rate_str, Point::new(72, 29));
    }

    fn draw_rate(&mut self, display: &mut Display, rate: &Rate) {
        let str = RateString::from(rate).0;
        display.draw_smol_text(&mut self.rate_str, str, Point::new(72, 29));
    }

    fn clear_length(&mut self, display: &mut Display) {
        display.clear_smol_text(&self.length_str, Point::new(74, 45));
    }

    fn draw_length(&mut self, display: &mut Display, length: &Length) {
        display.draw_smol_text(&mut self.length_str, length.0, Point::new(74, 45));
    }

    #[inline(always)]
    fn grid_point(&self, idx: usize, len: usize) -> Point {
        let x = idx % len % 8;
        let y = idx / 8;
        let p_x = GRID_START_X + x * (5 + 2);
        let p_y = GRID_START_Y + y * (5 + 5);
        Point::new(p_x as i32, p_y as i32)
    }

    fn clear_grid(&mut self, display: &mut Display) {
        for idx in 0..16 {
            display.clear_step_on(self.grid_point(idx, 16));
        }
    }

    fn draw_grid(&mut self, display: &mut Display, sequence: &Vec<bool, 16>) {
        let len = sequence.len();
        for idx in 0..len {
            let step_on = sequence[idx];
            let point = self.grid_point(idx, len);
            if step_on {
                display.draw_step_on(point);
            } else {
                display.draw_step_off(point);
            };
        }
    }

    pub fn draw_caret(&mut self, display: &mut Display, index: usize, len: usize) {
        let caret_point = |idx| -> Point {
            let mut grid_point = self.grid_point(idx, len);
            grid_point.x += 1;
            grid_point.y -= 3;
            grid_point
        };

        let idx = if index == 0 { len } else { index };
        display.clear_caret(caret_point(idx - 1));
        display.draw_caret(caret_point(index));
    }

    fn draw_output_type(&mut self, display: &mut Display, output_type: &OutputType) {
        let str = OutputTypeString::from(output_type).0;
        display.draw_bigge_text(&mut self.output_type_str, str, Point::new(0, 50));
    }

    fn draw_pointer(&mut self, display: &mut Display, element: &Element) {
        match element {
            Element::Rate => display.draw_pointer_right(Point::new(36, 10)),
            Element::Length => display.draw_pointer_right(Point::new(36, 28)),
            Element::Density => display.draw_pointer_right(Point::new(36, 46)),
            Element::OutputType => display.draw_pointer_left(Point::new(20, 25)),
            _ => {}
        };
    }
}
