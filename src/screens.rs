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

const POINTER_LEFT: &[u8; 630] = include_bytes!("assets/icons/pointer-left.bmp");
const POINTER_RIGHT: &[u8; 630] = include_bytes!("assets/icons/pointer-right.bmp");

enum Direction {
    Right,
    Left,
}

pub struct Screens {
    euclid: EuclidScreen,
    gate: GateScreen,
    home: HomeScreen,
    pointer_left: Bmp,
    pointer_right: Bmp,
}

impl Screens {
    pub fn new() -> Self {
        Self {
            euclid: EuclidScreen::new(),
            gate: GateScreen::new(),
            home: HomeScreen::new(),
            pointer_left: TinyBmp::from_slice(POINTER_LEFT).unwrap(),
            pointer_right: TinyBmp::from_slice(POINTER_RIGHT).unwrap(),
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
        let (point, dir) = match current {
            Element::Rate => (Point::new(36, 10), Direction::Right),
            Element::Prob => (Point::new(36, 28), Direction::Right),
            Element::Pwm => (Point::new(36, 46), Direction::Right),
            Element::Bpm => (Point::new(4, 8), Direction::Right),
            Element::Sync => (Point::new(4, 32), Direction::Right),
            Element::Length => (Point::new(36, 28), Direction::Right),
            Element::Density => (Point::new(36, 46), Direction::Right),
            Element::OutputType => (Point::new(20, 25), Direction::Left),
        };
        let pointer = match dir {
            Direction::Right => self.pointer_right,
            Direction::Left => self.pointer_left,
        };
        display.draw_bmp(&pointer, point);
    }
}
