use seq::OutputType;

use super::*;

#[derive(Clone)]
pub enum ScreenState {
    Home(Bpm, Sync, PlayStatus),
    Output(Output, OutputConfig, Option<usize>),
}

impl ScreenState {
    pub fn is_euclid(&self, current_output: Output) -> bool {
        if let ScreenState::Output(output, config, _) = self {
            if current_output == *output {
                config.output_type() == OutputType::Euclid
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn set_index(&mut self, index: usize) {
        if let ScreenState::Output(output, config, _) = self {
            *self = ScreenState::Output(*output, config.clone(), Some(index));
        }
    }
}
