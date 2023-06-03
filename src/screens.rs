use seq::OutputType;

use crate::{
    display::Display,
    screens::{euclid::EuclidScreen, gate::GateScreen, home::HomeScreen},
    state::ScreenState,
    StateChange,
};

mod euclid;
mod gate;
mod home;

pub struct Screens {
    euclid: EuclidScreen,
    gate: GateScreen,
    home: HomeScreen,
}

impl Screens {
    pub fn new() -> Self {
        Self {
            euclid: EuclidScreen::new(),
            gate: GateScreen::new(),
            home: HomeScreen::new(),
        }
    }

    pub fn draw(&mut self, state_change: StateChange, display: &mut Display) {
        match state_change {
            StateChange::Bpm(_) => self.home.draw(&state_change, display),
            StateChange::NextScreen(ref next_screen) => match next_screen {
                ScreenState::Home(..) => self.home.draw(&state_change, display),
                ScreenState::Output(_, config, _todo) => match config.output_type() {
                    OutputType::Euclid => self.euclid.draw(&state_change, config, display),
                    OutputType::Gate => self.gate.draw(&state_change, display),
                },
            },
            _ => {}
        }
    }
}
