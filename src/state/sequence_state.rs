use seq::{Density, Length};

use super::{Output, StateChange};

#[derive(Clone)]
pub struct SequenceState {
    pub density: Density,
    pub index: Option<usize>,
    pub length: Length,
    pub output: Output,
}

impl SequenceState {
    pub fn new(output: Output, length: Length, density: Density) -> Self {
        Self {
            density,
            index: Option::None,
            length,
            output,
        }
    }
}

impl From<SequenceState> for StateChange {
    fn from(val: SequenceState) -> Self {
        StateChange::Sequence(SequenceState {
            density: val.density,
            index: val.index,
            length: val.length,
            output: val.output,
        })
    }
}
