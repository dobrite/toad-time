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

    pub fn handle_state_change(&mut self, state_change: &StateChange) {
        match state_change {
            StateChange::Bpm(bpm) => self.bpm = *bpm,
            StateChange::Sync(sync) => self.sync = *sync,
            StateChange::Rate(output, rate) => self.outputs[usize::from(*output)].set_rate(*rate),
            StateChange::Pwm(output, pwm) => self.outputs[usize::from(*output)].set_pwm(*pwm),
            StateChange::Prob(output, prob) => self.outputs[usize::from(*output)].set_prob(*prob),
            StateChange::Length(output, length) => {
                self.outputs[usize::from(*output)].set_length(*length)
            }
            StateChange::Density(output, density) => {
                self.outputs[usize::from(*output)].set_density(*density)
            }
            StateChange::OutputType(output, output_type) => {
                let mut config = self.outputs[usize::from(*output)].clone();
                config.set_output_type(*output_type);
                self.current_screen = ScreenState::Output(*output, config, Option::None);
            }
            StateChange::Index(output, index) => {
                self.outputs[usize::from(*output)].set_index(*index)
            }
            StateChange::PlayStatus(play_status) => self.play_status = *play_status,
            StateChange::NextScreen(screen_state) => {
                self.current_screen = screen_state.clone();
                self.current_element = match self.current_screen {
                    ScreenState::Home(..) => Element::Bpm,
                    ScreenState::Output(..) => Element::Rate,
                };
            }
            StateChange::NextElement(element) => self.current_element = *element,
            StateChange::None => {}
        }
    }

    pub fn handle_command(&mut self, command: Command) -> StateChange {
        let current = self.current_element;

        match command {
            Command::EncoderRight => current.next(self),
            Command::EncoderLeft => current.prev(self),
            Command::EncoderPress => self.next_element(),
            Command::PagePress => self.next_screen(),
            Command::PlayPress => self.toggle_play(),
        }
    }

    fn next_screen(&mut self) -> StateChange {
        let next_screen = match self.current_screen {
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
                    self.outputs[usize::from(Output::B)].clone(),
                    Option::None,
                ),
                Output::C => ScreenState::Output(
                    Output::D,
                    self.outputs[usize::from(Output::B)].clone(),
                    Option::None,
                ),
                Output::D => ScreenState::Home(self.bpm, self.sync, self.play_status),
            },
        };

        StateChange::NextScreen(next_screen)
    }

    fn next_element(&mut self) -> StateChange {
        let next_element = match self.current_element {
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

        StateChange::NextElement(next_element)
    }

    fn toggle_play(&self) -> StateChange {
        let play_status = match self.play_status {
            PlayStatus::Playing => PlayStatus::Paused,
            PlayStatus::Paused => PlayStatus::Playing,
        };

        StateChange::PlayStatus(play_status)
    }
}
