use seq::{Density, Length, OutputType, Prob, Pwm, Rate, Seq};

use super::{Bpm, Element, Output, OutputScreenState, PlayStatus, Screen, ScreenState, Sync};

pub enum StateChange {
    Bpm(Bpm),
    Sync(Sync),
    Rate(Output, OutputType, Rate),
    Pwm(Output, Pwm),
    Prob(Output, Prob),
    Length(Output, Length, Density),
    Density(Output, Length, Density),
    OutputType(ScreenState),
    PlayStatus(Screen, PlayStatus),
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
            StateChange::PlayStatus(_, play_status) => match play_status {
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

    pub fn update_index(self, seq: &Seq) -> Self {
        match self {
            StateChange::OutputType(mut screen_state) => {
                let index = seq.get_index(screen_state.index().unwrap_or(0));
                screen_state.set_index(index);
                StateChange::OutputType(screen_state)
            }
            StateChange::NextScreen(mut screen_state) => {
                let index = seq.get_index(screen_state.index().unwrap_or(0));
                screen_state.set_index(index);
                StateChange::NextScreen(screen_state)
            }
            _ => self,
        }
    }
}
