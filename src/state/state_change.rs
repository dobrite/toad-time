use seq::{Density, Length, OutputType, Prob, Pwm, Rate};

use super::{Bpm, Element, Output, PlayStatus, Screen, Sync};

pub enum StateChange {
    Bpm(Bpm),
    Sync(Sync),
    Rate(Output, Rate),
    Pwm(Output, Pwm),
    Prob(Output, Prob),
    Length(Output, Length),
    Density(Output, Density),
    OutputType(Output, OutputType),
    PlayStatus(PlayStatus),
    NextScreen(Screen),
    NextElement(Element),
    Index(Output, u32),
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
