use seq::{OutputType, Prob, Pwm, Rate, Seq};

use super::*;

pub enum StateChange {
    Bpm(Bpm),
    Sync(Sync),
    Rate(Output, OutputType, Rate),
    Pwm(Output, Pwm),
    Prob(Output, Prob),
    Sequence(SequenceState),
    OutputType(ScreenState),
    PlayStatus(Screen, PlayStatus),
    NextScreen(ScreenState),
    NextElement(Screen, Element, Element),
    Index(Output, usize),
    Frame,
}

impl StateChange {
    pub fn update_seq(&self, seq: &mut Seq) {
        match self {
            StateChange::Bpm(bpm) => seq.set_bpm(bpm.0),
            StateChange::Sequence(SequenceState {
                output,
                length,
                density,
                ..
            }) => seq.set_sequence(output.into(), *length, *density),
            StateChange::OutputType(ScreenState::Output(OutputScreenState {
                output,
                ref config,
                ..
            })) => {
                seq.set_output_type(output.into(), config.output_type());
            }
            StateChange::PlayStatus(_, play_status) => match play_status {
                PlayStatus::Playing => { /* TODO: pause */ }
                PlayStatus::Paused => { /* TODO: reset then play */ }
            },
            StateChange::Prob(output, prob) => seq.set_prob(output.into(), *prob),
            StateChange::Pwm(output, pwm) => seq.set_pwm(output.into(), *pwm),
            StateChange::Rate(output, _, rate) => seq.set_rate(output.into(), *rate),
            StateChange::Frame
            | StateChange::Index(..)
            | StateChange::NextElement(..)
            | StateChange::NextScreen(..)
            | StateChange::OutputType(..)
            | StateChange::Sync(_) => {}
        }
    }

    pub fn update_index(self, seq: &Seq) -> Self {
        match self {
            StateChange::NextScreen(mut screen_state) => {
                let index = seq.get_index(screen_state.index().unwrap_or(0));
                screen_state.set_index(index);
                StateChange::NextScreen(screen_state)
            }
            StateChange::OutputType(mut screen_state) => {
                let index = seq.get_index(screen_state.index().unwrap_or(0));
                screen_state.set_index(index);
                StateChange::OutputType(screen_state)
            }
            StateChange::Sequence(mut sequence_state) => {
                let index = seq.get_index(sequence_state.output.into());
                sequence_state.index = Option::Some(index);
                StateChange::Sequence(sequence_state)
            }
            _ => self,
        }
    }
}

impl From<&StateChange> for Option<Screen> {
    fn from(val: &StateChange) -> Self {
        match val {
            StateChange::Frame => Option::None,
            StateChange::Bpm(_) | StateChange::Sync(_) => Option::Some(Screen::Home),
            StateChange::PlayStatus(screen, _) => {
                if let Screen::Home = screen {
                    Option::Some(Screen::Home)
                } else {
                    Option::None
                }
            }
            StateChange::Rate(output, output_type, _) => {
                Option::Some(Screen::Output(*output, *output_type))
            }
            StateChange::Prob(output, ..) | StateChange::Pwm(output, ..) => {
                Option::Some(Screen::Output(*output, OutputType::Gate))
            }
            StateChange::Index(output, ..)
            | StateChange::Sequence(SequenceState { output, .. }) => {
                Option::Some(Screen::Output(*output, OutputType::Euclid))
            }
            StateChange::OutputType(ref screen_state) => match screen_state {
                ScreenState::Output(OutputScreenState { output, config, .. }) => {
                    Option::Some(Screen::Output(*output, config.output_type()))
                }
                _ => unreachable!(),
            },
            StateChange::NextElement(screen, ..) => Option::Some(*screen),
            StateChange::NextScreen(ref next_screen) => match next_screen {
                ScreenState::Home(..) => Option::Some(Screen::Home),
                ScreenState::Output(OutputScreenState { output, config, .. }) => {
                    Option::Some(Screen::Output(*output, config.output_type()))
                }
            },
        }
    }
}
