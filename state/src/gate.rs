use core::fmt;

use defmt::Format;
use hash32::{Hash, Hasher};

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
