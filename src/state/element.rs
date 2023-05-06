use core::fmt;

use defmt::Format;
use hash32::{Hash, Hasher};

use super::{State, StateChange, Updatable};

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

#[derive(Clone, Copy, Eq, Format, PartialEq)]
pub enum Gate {
    A,
    B,
    C,
    D,
}

impl Hash for Gate {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Gate::A => state.write(&[0]),
            Gate::B => state.write(&[1]),
            Gate::C => state.write(&[2]),
            Gate::D => state.write(&[3]),
        }
    }
}

impl fmt::Display for Gate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::A => write!(f, "A"),
            Self::B => write!(f, "B"),
            Self::C => write!(f, "C"),
            Self::D => write!(f, "D"),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Home;
