use crate::{
    display::Display,
    state::{Element, HomeElement, StateChange, Sync},
};

mod gate;
mod home;

use gate::Gate;
use home::Home;

pub struct Screens {
    home: Home,
    gate_a: Gate,
    gate_b: Gate,
    gate_c: Gate,
    gate_d: Gate,
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
            gate_a: Gate::new("A"),
            gate_b: Gate::new("B"),
            gate_c: Gate::new("C"),
            gate_d: Gate::new("D"),
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
                Element::GateA(_) => {
                    self.state.current = page;
                    self.draw_gate_a(display);
                }
                Element::GateB(_) => {
                    self.state.current = page;
                    self.draw_gate_b(display);
                }
                Element::GateC(_) => {
                    self.state.current = page;
                    self.draw_gate_c(display);
                }
                Element::GateD(_) => {
                    self.state.current = page;
                    self.draw_gate_d(display);
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

    fn draw_gate_a(&mut self, display: &mut Display) {
        display.clear();
        self.gate_a.draw(&self.state, display);
        display.flush();
    }

    fn draw_gate_b(&mut self, display: &mut Display) {
        display.clear();
        self.gate_b.draw(&self.state, display);
        display.flush();
    }

    fn draw_gate_c(&mut self, display: &mut Display) {
        display.clear();
        self.gate_c.draw(&self.state, display);
        display.flush();
    }

    fn draw_gate_d(&mut self, display: &mut Display) {
        display.clear();
        self.gate_d.draw(&self.state, display);
        display.flush();
    }
}
