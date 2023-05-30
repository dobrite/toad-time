use seq::OutputType;

use super::*;

#[derive(Clone)]
pub struct State {
    pub bpm: Bpm,
    pub sync: Sync,
    pub play_status: PlayStatus,
    pub current_element: Element,
    pub current_screen: Screen,
    pub outputs: Outputs,
}

impl Default for State {
    fn default() -> Self {
        let mut outputs = Outputs::new();
        outputs.insert(Output::A, OutputConfig::new()).ok();
        outputs.insert(Output::B, OutputConfig::new()).ok();
        outputs.insert(Output::C, OutputConfig::new()).ok();
        outputs.insert(Output::D, OutputConfig::new()).ok();

        Self::new(outputs)
    }
}

impl State {
    pub fn new(outputs: Outputs) -> Self {
        Self {
            bpm: Bpm(120),
            sync: Sync::Ext,
            play_status: PlayStatus::Playing,
            current_element: Element::Bpm,
            current_screen: Screen::Home,
            outputs,
        }
    }

    pub fn handle_state_change(&mut self, state_change: &StateChange) {
        match state_change {
            StateChange::Bpm(bpm) => self.bpm = *bpm,
            StateChange::Sync(sync) => self.sync = *sync,
            StateChange::Rate(output, rate) => self.outputs[output].set_rate(*rate),
            StateChange::Pwm(output, pwm) => self.outputs[output].set_pwm(*pwm),
            StateChange::Prob(output, prob) => self.outputs[output].set_prob(*prob),
            StateChange::Length(output, length) => self.outputs[output].set_length(*length),
            StateChange::Density(output, density) => self.outputs[output].set_density(*density),
            StateChange::OutputType(output, output_type) => {
                self.outputs[output].set_output_type(*output_type);
                self.current_screen = Screen::Output(*output, *output_type);
            }
            StateChange::PlayStatus(play_status) => self.play_status = *play_status,
            StateChange::NextScreen(screen) => {
                self.current_screen = *screen;
                self.current_element = match screen {
                    Screen::Home => Element::Bpm,
                    Screen::Output(_, _) => Element::Rate,
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
            Screen::Home => Screen::Output(Output::A, self.outputs[&Output::A].output_type()),
            Screen::Output(Output::A, _) => {
                Screen::Output(Output::B, self.outputs[&Output::B].output_type())
            }
            Screen::Output(Output::B, _) => {
                Screen::Output(Output::C, self.outputs[&Output::C].output_type())
            }
            Screen::Output(Output::C, _) => {
                Screen::Output(Output::D, self.outputs[&Output::D].output_type())
            }
            Screen::Output(Output::D, _) => Screen::Home,
        };

        StateChange::NextScreen(next_screen)
    }

    fn next_element(&mut self) -> StateChange {
        let next_element = match self.current_element {
            Element::Bpm => Element::Sync,
            Element::Sync => Element::Bpm,
            Element::Rate => match self.current_screen {
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
