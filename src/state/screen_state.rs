use seq::OutputType;

use super::*;

#[derive(Clone)]
pub struct HomeScreenState {
    pub bpm: Bpm,
    pub sync: Sync,
    pub play_status: PlayStatus,
}

#[derive(Clone)]
pub struct OutputScreenState {
    pub output: Output,
    pub config: OutputConfig,
    pub index: Option<usize>,
}

#[derive(Clone)]
pub enum ScreenState {
    Home(HomeScreenState),
    Output(OutputScreenState),
}

impl ScreenState {
    pub fn new_home(bpm: Bpm, sync: Sync, play_status: PlayStatus) -> ScreenState {
        ScreenState::Home(HomeScreenState {
            bpm,
            sync,
            play_status,
        })
    }

    pub fn new_output(output: Output, config: OutputConfig, index: Option<usize>) -> ScreenState {
        ScreenState::Output(OutputScreenState {
            output,
            config,
            index,
        })
    }

    pub fn is_euclid(&self, current_output: Output) -> bool {
        if let ScreenState::Output(OutputScreenState { output, config, .. }) = self {
            if current_output == *output {
                config.output_type() == OutputType::Euclid
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn index(&self) -> Option<usize> {
        match self {
            ScreenState::Home(..) => Option::None,
            ScreenState::Output(OutputScreenState { output, .. }) => {
                Option::Some(usize::from(output))
            }
        }
    }

    pub fn set_index(&mut self, index: usize) {
        if let ScreenState::Output(OutputScreenState { output, config, .. }) = self {
            *self = ScreenState::new_output(*output, config.clone(), Some(index));
        }
    }
}
