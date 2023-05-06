use super::{Bpm, Element, Gate, PlayStatus, Prob, Pwm, Rate, Sync};

#[derive(PartialEq)]
pub enum StateChange {
    Initialize,
    Bpm(Bpm),
    Sync(Sync),
    Rate(Gate, Rate),
    Pwm(Gate, Pwm),
    Prob(Gate, Prob),
    PlayStatus(PlayStatus),
    NextPage(Element),
    NextElement(Element),
    None,
}

impl From<Option<Bpm>> for StateChange {
    fn from(val: Option<Bpm>) -> Self {
        match val {
            Option::Some(bpm) => StateChange::Bpm(bpm),
            Option::None => StateChange::None,
        }
    }
}

impl From<Option<Sync>> for StateChange {
    fn from(val: Option<Sync>) -> Self {
        match val {
            Option::Some(sync) => StateChange::Sync(sync),
            Option::None => StateChange::None,
        }
    }
}
