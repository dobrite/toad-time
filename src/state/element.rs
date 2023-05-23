use super::{Home, Output, State, StateChange, Updatable};

#[derive(Clone, Copy)]
pub enum Element {
    Rate(Output),
    Pwm(Output),
    Prob(Output),
    OutputType(Output),
    Bpm(Home),
    Sync(Home),
}

impl Element {
    pub fn next(&self, state: &mut State) -> StateChange {
        match self {
            Element::Bpm(Home) => state.bpm.next().into(),
            Element::Sync(Home) => state.sync.next().into(),
            Element::Rate(output) => match state.outputs[output].rate.next() {
                Option::Some(rate) => StateChange::Rate(*output, rate),
                Option::None => StateChange::None,
            },
            Element::Pwm(output) => match state.outputs[output].pwm.next() {
                Option::Some(pwm) => StateChange::Pwm(*output, pwm),
                Option::None => StateChange::None,
            },
            Element::Prob(output) => match state.outputs[output].prob.next() {
                Option::Some(prob) => StateChange::Prob(*output, prob),
                Option::None => StateChange::None,
            },
            Element::OutputType(output) => match state.outputs[output].output_type.next() {
                Option::Some(output_type) => StateChange::OutputType(*output, output_type),
                Option::None => StateChange::None,
            },
        }
    }

    pub fn prev(&self, state: &mut State) -> StateChange {
        match self {
            Element::Bpm(Home) => state.bpm.prev().into(),
            Element::Sync(Home) => state.sync.prev().into(),
            Element::Rate(output) => match state.outputs[output].rate.prev() {
                Option::Some(rate) => StateChange::Rate(*output, rate),
                Option::None => StateChange::None,
            },
            Element::Pwm(output) => match state.outputs[output].pwm.prev() {
                Option::Some(pwm) => StateChange::Pwm(*output, pwm),
                Option::None => StateChange::None,
            },
            Element::Prob(output) => match state.outputs[output].prob.prev() {
                Option::Some(prob) => StateChange::Prob(*output, prob),
                Option::None => StateChange::None,
            },
            Element::OutputType(output) => match state.outputs[output].output_type.prev() {
                Option::Some(output_type) => StateChange::OutputType(*output, output_type),
                Option::None => StateChange::None,
            },
        }
    }
}
