use embedded_graphics::prelude::Point;
use tinybmp::Bmp as TinyBmp;

use crate::{
    display::{Bmp, Display},
    screens::{gate::GateScreen, home::HomeScreen},
    state::{Element, Output, State},
};

mod gate;
mod home;

const POINTER: &[u8; 630] = include_bytes!("../assets/icons/Pointer.bmp");

pub struct Screens {
    gate: GateScreen,
    home: HomeScreen,
    pointer: Bmp,
}

impl Screens {
    pub fn new() -> Self {
        Self {
            gate: GateScreen::new(),
            home: HomeScreen::new(),
            pointer: TinyBmp::from_slice(POINTER).unwrap(),
        }
    }

    pub async fn draw(&mut self, state: &State, display: &mut Display) {
        match state.current_element {
            Element::Prob(output) | Element::Pwm(output) | Element::Rate(output) => {
                self.draw_output(&output, state, display).await
            }
            Element::Bpm(_) | Element::Sync(_) => self.draw_home(state, display).await,
        }
    }

    pub async fn draw_home(&mut self, state: &State, display: &mut Display) {
        display.clear();
        self.home.draw(state, display);
        self.draw_pointer(state.current_element, display);
        display.flush().await;
    }

    async fn draw_output(&mut self, output: &Output, state: &State, display: &mut Display) {
        display.clear();
        let output_config = &state.outputs[output];
        self.gate.draw(output, output_config, display);
        self.draw_pointer(state.current_element, display);
        display.flush().await;
    }

    fn draw_pointer(&mut self, current: Element, display: &mut Display) {
        let point = match current {
            Element::Rate(_) => Point::new(36, 10),
            Element::Pwm(_) => Point::new(36, 28),
            Element::Prob(_) => Point::new(36, 46),
            Element::Bpm(_) => Point::new(4, 8),
            Element::Sync(_) => Point::new(4, 32),
        };
        display.draw_bmp(&self.pointer, point);
    }
}
