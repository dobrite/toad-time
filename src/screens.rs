use seq::OutputType;

use crate::{
    display::Display,
    screens::{euclid::EuclidScreen, gate::GateScreen, home::HomeScreen},
    state::{OutputScreenState, Screen, ScreenState},
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
            StateChange::Sync(_) => self.home.draw(&state_change, display),
            StateChange::PlayStatus(screen, _) => {
                if let Screen::Home = screen {
                    self.home.draw(&state_change, display)
                }
            }
            StateChange::Rate(_, output_type, _) => match output_type {
                OutputType::Gate => self.gate.draw(&state_change, display),
                OutputType::Euclid => self.euclid.draw(&state_change, display),
            },
            StateChange::Pwm(..) => self.gate.draw(&state_change, display),
            StateChange::Prob(..) => self.gate.draw(&state_change, display),
            StateChange::Length(..) => self.euclid.draw(&state_change, display),
            StateChange::Density(..) => self.euclid.draw(&state_change, display),
            StateChange::Index(..) => self.euclid.draw(&state_change, display),
            StateChange::OutputType(ref screen_state) => match screen_state {
                ScreenState::Output(OutputScreenState { config, .. }) => match config.output_type()
                {
                    OutputType::Euclid => self.euclid.draw(&state_change, display),
                    OutputType::Gate => self.gate.draw(&state_change, display),
                },
                _ => unreachable!(),
            },
            StateChange::NextElement(ref screen_state, _) => match screen_state {
                ScreenState::Home(..) => self.home.draw(&state_change, display),
                ScreenState::Output(OutputScreenState { config, .. }) => match config.output_type()
                {
                    OutputType::Euclid => self.euclid.draw(&state_change, display),
                    OutputType::Gate => self.gate.draw(&state_change, display),
                },
            },
            StateChange::NextScreen(ref next_screen) => match next_screen {
                ScreenState::Home(..) => self.home.draw(&state_change, display),
                ScreenState::Output(OutputScreenState { config, .. }) => match config.output_type()
                {
                    OutputType::Euclid => self.euclid.draw(&state_change, display),
                    OutputType::Gate => self.gate.draw(&state_change, display),
                },
            },
        }
    }
}
