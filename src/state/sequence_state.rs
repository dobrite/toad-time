use seq::{Density, Length};

use super::Output;

#[derive(Clone)]
pub struct SequenceState {
    pub density: Density,
    pub index: Option<usize>,
    pub length: Length,
    pub output: Output,
}
