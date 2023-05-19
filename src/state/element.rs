use super::{Gate, Home, State, StateChange, Updatable};

#[derive(Clone, Copy)]
pub enum Element {
    Rate(Gate),
    Pwm(Gate),
    Prob(Gate),
    Bpm(Home),
    Sync(Home),
}

impl Element {
    pub fn next(&self, state: &mut State) -> StateChange {
        match self {
            Element::Bpm(Home) => state.bpm.next().into(),
            Element::Sync(Home) => state.sync.next().into(),
            Element::Rate(gate) => match state.gates[gate].rate.next() {
                Option::Some(rate) => StateChange::Rate(*gate, rate),
                Option::None => StateChange::None,
            },
            Element::Pwm(gate) => match state.gates[gate].pwm.next() {
                Option::Some(pwm) => StateChange::Pwm(*gate, pwm),
                Option::None => StateChange::None,
            },
            Element::Prob(gate) => match state.gates[gate].prob.next() {
                Option::Some(prob) => StateChange::Prob(*gate, prob),
                Option::None => StateChange::None,
            },
        }
    }

    pub fn prev(&self, state: &mut State) -> StateChange {
        match self {
            Element::Bpm(Home) => state.bpm.prev().into(),
            Element::Sync(Home) => state.sync.prev().into(),
            Element::Rate(gate) => match state.gates[gate].rate.prev() {
                Option::Some(rate) => StateChange::Rate(*gate, rate),
                Option::None => StateChange::None,
            },
            Element::Pwm(gate) => match state.gates[gate].pwm.prev() {
                Option::Some(pwm) => StateChange::Pwm(*gate, pwm),
                Option::None => StateChange::None,
            },
            Element::Prob(gate) => match state.gates[gate].prob.prev() {
                Option::Some(prob) => StateChange::Prob(*gate, prob),
                Option::None => StateChange::None,
            },
        }
    }
}
