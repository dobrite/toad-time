use core::fmt;

use defmt::Format;

use super::Updatable;

#[derive(Clone, Copy, PartialEq, Format)]
pub enum Sync {
    Int,
    Ext,
}

impl Updatable for Sync {
    fn next(&mut self) -> Option<Self> {
        if *self == Sync::Int {
            Option::None
        } else {
            *self = Sync::Int;
            Option::Some(*self)
        }
    }

    fn prev(&mut self) -> Option<Self> {
        if *self == Sync::Ext {
            Option::None
        } else {
            *self = Sync::Ext;
            Option::Some(*self)
        }
    }
}

impl fmt::Display for Sync {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int => write!(f, "Int"),
            Self::Ext => write!(f, "Ext"),
        }
    }
}
