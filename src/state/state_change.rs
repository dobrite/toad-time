use seq::{Density, Length, OutputType, Prob, Pwm, Rate};

use super::{Bpm, Element, Output, PlayStatus, ScreenState, Sync};

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
