use crate::{
    display::Display,
    state::{Element, Gate, HomeElement, StateChange, Sync},
};

mod gate;
mod home;

pub struct Screens {
    home: home::Home,
    gate_a: gate::Gate,
    gate_b: gate::Gate,
    gate_c: gate::Gate,
    gate_d: gate::Gate,
    state: ScreenState,
}

pub struct ScreenState {
    bpm: u32,
    sync: Sync,
    is_playing: bool,
    current: Element,
}

impl Default for ScreenState {
    fn default() -> Self {
        Self::new()
    }
}

impl ScreenState {
    pub fn new() -> Self {
        Self {
            bpm: 120,
            sync: Sync::Ext,
            is_playing: true,
            current: Element::Home(HomeElement::Bpm),
        }
    }
}

impl Screens {
    pub fn new() -> Self {
        Self {
            home: Default::default(),
            gate_a: gate::Gate::new("A"),
            gate_b: gate::Gate::new("B"),
            gate_c: gate::Gate::new("C"),
            gate_d: gate::Gate::new("D"),
            state: Default::default(),
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
            StateChange::NextPage(page) => match page {
                Element::Home(_) => {
                    self.state.current = page;
                    self.draw_home(display);
                }
                Element::Gate(gate, _) => {
                    self.state.current = page;
                    self.draw_gate(gate, display);
                }
            },
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