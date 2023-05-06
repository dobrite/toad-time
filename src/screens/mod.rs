use embedded_graphics::prelude::Point;
use tinybmp::Bmp as TinyBmp;

use crate::{
    display::{Bmp, Display},
    screens::{gate::GateScreen, home::HomeScreen},
    state::{Element, Gate, State, StateChange},
};

mod gate;
mod home;

const POINTER: &[u8; 630] = include_bytes!("../assets/icons/Pointer.bmp");

pub struct Screens {
    gate_a: GateScreen,
    gate_b: GateScreen,
    gate_c: GateScreen,
    gate_d: GateScreen,
    home: HomeScreen,
    pointer: Bmp,
}

impl Screens {
    pub fn new() -> Self {
        Self {
            gate_a: GateScreen::new(Gate::A),
            gate_b: GateScreen::new(Gate::B),
            gate_c: GateScreen::new(Gate::C),
            gate_d: GateScreen::new(Gate::D),
            home: HomeScreen::new(),
            pointer: TinyBmp::from_slice(POINTER).unwrap(),
        }
    }

    pub fn draw(&mut self, state: &State, state_change: &StateChange, display: &mut Display) {
        match state_change {
            StateChange::Bpm(_)
            | StateChange::Initialize
            | StateChange::PlayStatus(_)
            | StateChange::Sync(_) => self.draw_home(state, display),
            StateChange::Rate(gate, _) | StateChange::Pwm(gate, _) | StateChange::Prob(gate, _) => {
                self.draw_gate(gate, state, display)
            }
            StateChange::NextPage(element) | StateChange::NextElement(element) => match element {
                Element::Bpm(_) | Element::Sync(_) => {
                    self.draw_home(state, display);
                }
                Element::Pwm(gate) | Element::Rate(gate) | Element::Prob(gate) => {
                    self.draw_gate(gate, state, display);
                }
            },
            StateChange::None => unreachable!(),
        }
    }

    fn draw_home(&mut self, state: &State, display: &mut Display) {
        display.clear();
        self.home.draw(state, display);
        self.draw_pointer(state.current, display);
        display.flush();
    }

    fn draw_gate(&mut self, gate: &Gate, state: &State, display: &mut Display) {
        display.clear();
        let gate_state = &state.gates[gate];
        match gate {
            Gate::A => self.gate_a.draw(gate_state, display),
            Gate::B => self.gate_b.draw(gate_state, display),
            Gate::C => self.gate_c.draw(gate_state, display),
            Gate::D => self.gate_d.draw(gate_state, display),
        }
        self.draw_pointer(state.current, display);
        display.flush();
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
