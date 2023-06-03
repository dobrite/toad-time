use seq::OutputType;

use crate::{
    display::Display,
    screens::{euclid::EuclidScreen, gate::GateScreen, home::HomeScreen},
    state::{Output, ScreenState, State},
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

    pub fn draw(&mut self, state_change: StateChange, state: &State, display: &mut Display) {
        match state.current_screen {
            ScreenState::Home(..) => self.home.draw(&state_change, state, display),
            ScreenState::Output(output, ..) => {
                self.draw_output(&output, &state_change, state, display)
            }
        }
    }

    fn draw_output(
        &mut self,
        output: &Output,
        state_change: &StateChange,
        state: &State,
        display: &mut Display,
    ) {
        let output_config = &state.outputs[usize::from(*output)];
        match output_config.output_type() {
            OutputType::Euclid => self.euclid.draw(state_change, output_config, display),
            OutputType::Gate => self.gate.draw(state_change, output_config, display),
        }
    }
}
