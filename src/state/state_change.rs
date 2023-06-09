use heapless::Vec;
use seq::{Density, Length, OutputType, Prob, Pwm, Rate, Seq};

use super::{Bpm, Element, Output, OutputScreenState, PlayStatus, ScreenState, Sync};

pub enum StateChange {
    Bpm(Bpm),
    Sync(Sync),
    Rate(Output, OutputType, Rate),
    Pwm(Output, Pwm),
    Prob(Output, Prob),
    Length(Output, Length, Density),
    Density(Output, Length, Density),
    OutputType(ScreenState),
    PlayStatus(PlayStatus),
    NextScreen(ScreenState),
    NextElement(ScreenState, Element),
    Index(Output, usize),
}

impl StateChange {
    pub fn update_seq(&self, seq: &mut Seq) {
        match self {
            StateChange::Bpm(bpm) => seq.set_bpm(bpm.0),
            StateChange::Density(output, _, density) => seq.set_density(output.into(), *density),
            StateChange::Length(output, length, _) => seq.set_length(output.into(), *length),
            StateChange::OutputType(ScreenState::Output(OutputScreenState {
                output,
                ref config,
                ..
            })) => {
                seq.set_output_type(output.into(), config.output_type());
            }
            StateChange::PlayStatus(play_status) => match play_status {
                PlayStatus::Playing => { /* TODO: pause */ }
                PlayStatus::Paused => { /* TODO: reset then play */ }
            },
            StateChange::Prob(output, prob) => seq.set_prob(output.into(), *prob),
            StateChange::Pwm(output, pwm) => seq.set_pwm(output.into(), *pwm),
            StateChange::Rate(output, _, rate) => seq.set_rate(output.into(), *rate),
            StateChange::Index(..)
            | StateChange::NextElement(..)
            | StateChange::NextScreen(..)
            | StateChange::OutputType(..)
            | StateChange::Sync(_) => {}
        }
    }

    pub fn update_index(&mut self, state_changes: &Vec<StateChange, 4>) {
        if let Some(StateChange::Index(_, index)) = state_changes.first() {
            match self {
                StateChange::OutputType(screen_state) => {
                    screen_state.set_index(*index);
                    *self = StateChange::OutputType(screen_state.clone())
                }
                StateChange::NextScreen(screen_state) => {
                    screen_state.set_index(*index);
                    *self = StateChange::NextScreen(screen_state.clone())
                }
                _ => {}
            }
        };
    }
}
