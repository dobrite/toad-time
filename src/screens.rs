use embedded_graphics::prelude::Point;
use tinybmp::Bmp as TinyBmp;

use crate::{
    display::{Bmp, Display},
    screens::{gate::GateScreen, home::HomeScreen},
    state::{Element, Gate, State, StateChange},
};

mod gate;
mod home;

const POINTER: &[u8; 630] = include_bytes!("assets/icons/Pointer.bmp");

pub struct Screens {
    gate_a: GateScreen,
    gate_b: GateScreen,
    gate_c: GateScreen,
    gate_d: GateScreen,
    home: HomeScreen,
    pointer: Bmp,
    state: State,
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
            state: State::new(),
        }
    }

    pub fn handle_state_change(&mut self, state_change: StateChange, display: &mut Display) {
        match state_change {
            StateChange::Initialize => {
                self.draw_home(display);
            }
            StateChange::Bpm(bpm) => {
                self.state.bpm = bpm;
                self.draw_home(display);
            }
            StateChange::NextPage(element) | StateChange::NextElement(element) => match element {
                Element::Bpm(_) | Element::Sync(_) => {
                    self.state.current = element;
                    self.draw_home(display);
                }
                Element::Pwm(gate) | Element::Rate(gate) | Element::Prob(gate) => {
                    self.state.current = element;
                    self.draw_gate(gate, display);
                }
            },
            StateChange::Sync(sync) => {
                self.state.sync = sync;
                self.draw_home(display);
            }
            StateChange::PlayStatus(play_status) => {
                self.state.play_status = play_status;
                self.draw_home(display);
            }
            StateChange::Rate(gate, rate) => {
                self.state.gates[&gate].rate = rate;
                self.draw_gate(gate, display);
            }
            StateChange::Pwm(gate, pwm) => {
                self.state.gates[&gate].pwm = pwm;
                self.draw_gate(gate, display);
            }
            StateChange::Prob(gate, prob) => {
                self.state.gates[&gate].prob = prob;
                self.draw_gate(gate, display);
            }
            StateChange::None => unreachable!(),
        }
    }

    fn draw_home(&mut self, display: &mut Display) {
        display.clear();
        self.home.draw(&self.state, display);
        self.draw_pointer(self.state.current, display);
        display.flush();
    }

    fn draw_gate(&mut self, gate: Gate, display: &mut Display) {
        display.clear();
        match gate {
            Gate::A => self.gate_a.draw(&self.state.gates[&gate], display),
            Gate::B => self.gate_b.draw(&self.state.gates[&gate], display),
            Gate::C => self.gate_c.draw(&self.state.gates[&gate], display),
            Gate::D => self.gate_d.draw(&self.state.gates[&gate], display),
        }
        self.draw_pointer(self.state.current, display);
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
