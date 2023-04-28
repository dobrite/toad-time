use crate::{
    display::Display,
    state::{Element, Gate, State, StateChange},
};

mod gate;
mod home;

const POINTER: &[u8; 630] = include_bytes!("../assets/icons/Pointer.bmp");

pub struct Screens {
    home: home::Home,
    gate_a: gate::Gate,
    gate_b: gate::Gate,
    gate_c: gate::Gate,
    gate_d: gate::Gate,
    state: State,
}

impl Screens {
    pub fn new() -> Self {
        Self {
            home: Default::default(),
            gate_a: gate::Gate::new(Gate::A),
            gate_b: gate::Gate::new(Gate::B),
            gate_c: gate::Gate::new(Gate::C),
            gate_d: gate::Gate::new(Gate::D),
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
                Element::Pwm(gate) | Element::Rate(gate) => {
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
            StateChange::None => unreachable!(),
        }
    }

    fn draw_home(&mut self, display: &mut Display) {
        display.clear();
        self.home.draw(&self.state, display);
        display.flush();
    }

    fn draw_gate(&mut self, gate: Gate, display: &mut Display) {
        display.clear();
        match gate {
            Gate::A => self.gate_a.draw(&self.state, display),
            Gate::B => self.gate_b.draw(&self.state, display),
            Gate::C => self.gate_c.draw(&self.state, display),
            Gate::D => self.gate_d.draw(&self.state, display),
        }
        display.flush();
    }
}
