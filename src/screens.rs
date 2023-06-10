use seq::OutputType;

use crate::{
    display::Display,
    screens::{euclid::EuclidScreen, gate::GateScreen, home::HomeScreen},
    state::Screen,
    StateChange,
};

mod euclid;
mod gate;
mod home;

pub struct Screens {
    euclid: EuclidScreen,
    gate: GateScreen,
    home: HomeScreen,
    current_screen: Screen,
}

impl Screens {
    pub fn new() -> Self {
        Self {
            euclid: EuclidScreen::new(),
            gate: GateScreen::new(),
            home: HomeScreen::new(),
            current_screen: Screen::Home,
        }
    }

    pub fn draw(&mut self, state_change: StateChange, display: &mut Display) {
        if let Some(screen) = Option::<Screen>::from(&state_change) {
            self.current_screen = screen;
        };

        match self.current_screen {
            Screen::Home => self.home.draw(state_change, display),
            Screen::Output(_, output_type) => match output_type {
                OutputType::Gate => self.gate.draw(state_change, display),
                OutputType::Euclid => self.euclid.draw(state_change, display),
            },
        }
    }
}
