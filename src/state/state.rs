use heapless::Vec;
use seq::OutputType;

use super::*;

#[derive(Clone)]
pub struct State {
    pub bpm: Bpm,
    pub bpm_sync: Option<BpmSync>,
    pub sync: Sync,
    pub play_status: PlayStatus,
    pub current_element: Element,
    pub current_screen: Screen,
    pub outputs: Vec<OutputConfig, 4>,
}

impl Default for State {
    fn default() -> Self {
        let mut outputs = Vec::new();
        outputs.push(OutputConfig::new()).ok();
        outputs.push(OutputConfig::new()).ok();
        outputs.push(OutputConfig::new()).ok();
        outputs.push(OutputConfig::new()).ok();

        Self::new(outputs)
    }
}

impl State {
    pub fn new(outputs: Vec<OutputConfig, 4>) -> Self {
        Self {
            bpm: Bpm(120),
            bpm_sync: Option::Some(BpmSync::new()),
            sync: Sync::Int,
            play_status: PlayStatus::Playing,
            current_element: Element::Bpm,
            current_screen: Screen::Home,
            outputs,
        }
    }

    pub fn handle_command(&mut self, command: Command) -> Option<StateChange> {
        let current = &mut self.current_element.clone();

        match command {
            Command::EncoderRight => current.next(self).map(|state_change| match state_change {
                StateChange::OutputType(ref screen_state) => {
                    self.current_screen = screen_state.into();
                    state_change
                }
                _ => state_change,
            }),
            Command::EncoderLeft => current.prev(self).map(|state_change| match state_change {
                StateChange::OutputType(ref screen_state) => {
                    self.current_screen = screen_state.into();
                    state_change
                }
                _ => state_change,
            }),
            Command::EncoderPress => Some(self.next_element()),
            Command::PagePress => Some(self.next_screen()),
            Command::PlayPress => Some(self.toggle_play()),
            Command::BpmPress => self.bpm_sync(),
        }
    }

    fn next_screen(&mut self) -> StateChange {
        self.current_screen = match self.current_screen {
            Screen::Home => Screen::Output(
                Output::A,
                self.outputs[usize::from(Output::A)].output_type(),
            ),
            Screen::Output(Output::A, _) => Screen::Output(
                Output::B,
                self.outputs[usize::from(Output::B)].output_type(),
            ),
            Screen::Output(Output::B, _) => Screen::Output(
                Output::C,
                self.outputs[usize::from(Output::C)].output_type(),
            ),
            Screen::Output(Output::C, _) => Screen::Output(
                Output::D,
                self.outputs[usize::from(Output::D)].output_type(),
            ),
            Screen::Output(Output::D, _) => Screen::Home,
        };
        self.current_element = match self.current_screen {
            Screen::Home => Element::Bpm,
            Screen::Output(..) => Element::Rate,
        };

        StateChange::NextScreen(self.to_screen_state())
    }

    fn next_element(&mut self) -> StateChange {
        let prev_element = self.current_element.clone();

        self.current_element = match self.current_element {
            Element::Bpm => Element::Sync,
            Element::Sync => Element::Bpm,
            Element::Rate => match &self.current_screen {
                Screen::Home => unreachable!(),
                Screen::Output(_, output_type) => match output_type {
                    OutputType::Gate => Element::Prob,
                    OutputType::Euclid => Element::Length,
                },
            },
            Element::Length => Element::Density,
            Element::Density => Element::OutputType,
            Element::Prob => Element::Pwm,
            Element::Pwm => Element::OutputType,
            Element::OutputType => Element::Rate,
        };

        StateChange::NextElement(
            self.current_screen,
            prev_element,
            self.current_element.clone(),
        )
    }

    fn to_screen_state(&self) -> ScreenState {
        match self.current_screen {
            Screen::Home => ScreenState::Home(HomeScreenState {
                bpm: self.bpm,
                sync: self.sync,
                play_status: self.play_status,
            }),
            Screen::Output(output, _) => {
                let config = self.outputs[usize::from(output)].clone();
                ScreenState::Output(OutputScreenState {
                    output,
                    config,
                    index: Option::None,
                })
            }
        }
    }

    fn toggle_play(&mut self) -> StateChange {
        self.play_status = match self.play_status {
            PlayStatus::Playing => PlayStatus::Paused,
            PlayStatus::Paused => PlayStatus::Playing,
        };

        StateChange::PlayStatus(self.current_screen, self.play_status)
    }

    fn bpm_sync(&mut self) -> Option<StateChange> {
        if let Sync::Int = self.sync {
            return Option::None
        }

        match &mut self.bpm_sync {
            Option::None => {
                self.bpm_sync = Option::Some(BpmSync::new());
                Option::None
            }
            Option::Some(bpm_sync) => bpm_sync.pulse().map(|next_bpm| {
                self.bpm = Bpm(next_bpm);
                StateChange::Bpm(self.bpm)
            }),
        }
    }
}
