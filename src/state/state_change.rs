use super::{Bpm, Element, Output, OutputType, PlayStatus, Prob, Pwm, Rate, Sync};

pub enum StateChange {
    Bpm(Bpm),
    Sync(Sync),
    Rate(Output, Rate),
    Pwm(Output, Pwm),
    Prob(Output, Prob),
    OutputType(Output, OutputType),
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
