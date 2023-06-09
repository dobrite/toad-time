use core::fmt;

use defmt::Format;

#[derive(Clone, Copy, Eq, Format, PartialEq)]
pub enum Output {
    A,
    B,
    C,
    D,
}

impl Output {
    pub fn into_output(idx: usize) -> Output {
        match idx {
            0 => Output::A,
            1 => Output::B,
            2 => Output::C,
            3 => Output::D,
            _ => unreachable!(),
        }
    }
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

impl From<&Output> for usize {
    fn from(val: &Output) -> Self {
        match val {
            Output::A => 0,
            Output::B => 1,
            Output::C => 2,
            Output::D => 3,
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
