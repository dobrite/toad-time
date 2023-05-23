use embedded_graphics::prelude::Point;
use seq::OutputType;
use tinybmp::Bmp as TinyBmp;

use crate::{
    display::{Bmp, Display},
    screens::{euclid::EuclidScreen, gate::GateScreen, home::HomeScreen},
    state::{Element, Output, Screen, State},
};

mod euclid;
mod gate;
mod home;

const POINTER: &[u8; 630] = include_bytes!("../assets/icons/Pointer.bmp");

pub struct Screens {
    euclid: EuclidScreen,
    gate: GateScreen,
    home: HomeScreen,
    pointer: Bmp,
}

impl Screens {
    pub fn new() -> Self {
        Self {
            euclid: EuclidScreen::new(),
            gate: GateScreen::new(),
            home: HomeScreen::new(),
            pointer: TinyBmp::from_slice(POINTER).unwrap(),
        }
    }

    pub async fn draw(&mut self, state: &State, display: &mut Display) {
        match state.current_screen {
            Screen::Home => self.draw_home(state, display).await,
            Screen::Output(output, _) => self.draw_output(&output, state, display).await,
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
        match output_config.output_type {
            OutputType::Euclid => self.euclid.draw(output, output_config, display),
            OutputType::Gate => self.gate.draw(output, output_config, display),
        }
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
            Element::OutputType(_) => todo!(),
        };
        display.draw_bmp(&self.pointer, point);
    }
}
