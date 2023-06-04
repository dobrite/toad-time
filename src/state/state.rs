use heapless::Vec;
use seq::OutputType;

use super::*;

#[derive(Clone)]
pub struct State {
    pub bpm: Bpm,
    pub sync: Sync,
    pub play_status: PlayStatus,
    pub current_element: Element,
    pub current_screen: ScreenState,
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
            sync: Sync::Ext,
            play_status: PlayStatus::Playing,
            current_element: Element::Bpm,
            current_screen: ScreenState::Home(Bpm(120), Sync::Ext, PlayStatus::Playing),
            outputs,
        }
    }

    pub fn handle_command(&mut self, command: Command) -> Option<StateChange> {
        let current = self.current_element;

        match command {
            Command::EncoderRight => current.next(self).map(|state_change| match state_change {
                StateChange::OutputType(ref screen_state) => {
                    self.current_screen = screen_state.clone();
                    state_change
                }
                _ => state_change,
            }),
            Command::EncoderLeft => current.prev(self).map(|state_change| match state_change {
                StateChange::OutputType(ref screen_state) => {
                    self.current_screen = screen_state.clone();
                    state_change
                }
                _ => state_change,
            }),
            Command::EncoderPress => Some(self.next_element()),
            Command::PagePress => Some(self.next_screen()),
            Command::PlayPress => Some(self.toggle_play()),
        }
    }

    fn next_screen(&mut self) -> StateChange {
        self.current_screen = match self.current_screen {
            ScreenState::Home(_, _, _) => ScreenState::Output(
                Output::A,
                self.outputs[usize::from(Output::A)].clone(),
                Option::None,
            ),
            ScreenState::Output(output, ..) => match output {
                Output::A => ScreenState::Output(
                    Output::B,
                    self.outputs[usize::from(Output::B)].clone(),
                    Option::None,
                ),
                Output::B => ScreenState::Output(
                    Output::C,
                    self.outputs[usize::from(Output::C)].clone(),
                    Option::None,
                ),
                Output::C => ScreenState::Output(
                    Output::D,
                    self.outputs[usize::from(Output::D)].clone(),
                    Option::None,
                ),
                Output::D => ScreenState::Home(self.bpm, self.sync, self.play_status),
            },
        };
        self.current_element = match self.current_screen {
            ScreenState::Home(..) => Element::Bpm,
            ScreenState::Output(..) => Element::Rate,
        };

        StateChange::NextScreen(self.current_screen.clone())
    }

    fn next_element(&mut self) -> StateChange {
        self.current_element = match self.current_element {
            Element::Bpm => Element::Sync,
            Element::Sync => Element::Bpm,
            Element::Rate => match &self.current_screen {
                ScreenState::Home(..) => unreachable!(),
                ScreenState::Output(_, config, _) => match config.output_type() {
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

        StateChange::NextElement(self.current_screen.clone(), self.current_element)
    }

    fn toggle_play(&mut self) -> StateChange {
        self.play_status = match self.play_status {
            PlayStatus::Playing => PlayStatus::Paused,
            PlayStatus::Paused => PlayStatus::Playing,
        };

        StateChange::PlayStatus(self.play_status)
    }
}
