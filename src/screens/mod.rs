use embedded_graphics::prelude::Point;
use tinybmp::Bmp as TinyBmp;

use crate::{
    display::{Bmp, Display},
    screens::{gate::GateScreen, home::HomeScreen},
    state::{Element, Output, State, StateChange},
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

    pub async fn draw(&mut self, state: &State, state_change: &StateChange, display: &mut Display) {
        match state_change {
            StateChange::Bpm(_) | StateChange::Sync(_) => self.draw_home(state, display).await,
            StateChange::Rate(output, _)
            | StateChange::Pwm(output, _)
            | StateChange::Prob(output, _) => self.draw_gate(output, state, display).await,
            StateChange::NextPage(element) | StateChange::NextElement(element) => match element {
                Element::Bpm(_) | Element::Sync(_) => self.draw_home(state, display).await,
                Element::Pwm(output) | Element::Rate(output) | Element::Prob(output) => {
                    self.draw_gate(output, state, display).await
                }
            },
            StateChange::PlayStatus(_) => match state.current_element {
                Element::Bpm(_) | Element::Sync(_) => self.draw_home(state, display).await,
                Element::Prob(_) | Element::Pwm(_) | Element::Rate(_) => {}
            },
            StateChange::None => unreachable!(),
        }
    }

    pub async fn draw_home(&mut self, state: &State, display: &mut Display) {
        display.clear();
        self.home.draw(state, display);
        self.draw_pointer(state.current_element, display);
        display.flush().await;
    }

    async fn draw_gate(&mut self, output: &Output, state: &State, display: &mut Display) {
        display.clear();
        let gate_state = &state.outputs[output];
        self.gate.draw(output, gate_state, display);
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
