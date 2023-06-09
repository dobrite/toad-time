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

impl From<ScreenState> for Screen {
    fn from(val: ScreenState) -> Self {
        match val {
            ScreenState::Home(..) => Screen::Home,
            ScreenState::Output(OutputScreenState { output, config, .. }) => {
                let output_type = config.output_type();
                Screen::Output(output, output_type)
            }
        }
    }
}

impl From<&ScreenState> for Screen {
    fn from(val: &ScreenState) -> Self {
        match val {
            ScreenState::Home(..) => Screen::Home,
            ScreenState::Output(OutputScreenState { output, config, .. }) => {
                let output_type = config.output_type();
                Screen::Output(*output, output_type)
            }
        }
    }
}
