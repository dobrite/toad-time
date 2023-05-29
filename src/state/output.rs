use core::fmt;

use defmt::Format;
use hash32::{Hash, Hasher};

#[derive(Clone, Copy, Eq, Format, PartialEq)]
pub enum Output {
    A,
    B,
    C,
    D,
}

impl From<Output> for usize {
    fn from(val: Output) -> Self {
        match val {
            Output::A => 0,
            Output::B => 1,
            Output::C => 2,
            Output::D => 3,
        }
    }
}

impl Hash for Output {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Output::A => state.write(&[0]),
            Output::B => state.write(&[1]),
            Output::C => state.write(&[2]),
            Output::D => state.write(&[3]),
        }
    }
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::A => write!(f, "A"),
            Self::B => write!(f, "B"),
            Self::C => write!(f, "C"),
            Self::D => write!(f, "D"),
        }
    }
}
