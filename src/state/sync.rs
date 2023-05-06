use core::fmt;

use defmt::Format;

use super::Updatable;

#[derive(Clone, Copy, PartialEq, Format)]
pub enum Sync {
    Int,
    Ext,
}

impl Updatable for Sync {
    fn next(&self) -> Option<Self> {
        if *self == Sync::Int {
            Option::None
        } else {
            Option::Some(Sync::Int)
        }
    }

    fn prev(&self) -> Option<Self> {
        if *self == Sync::Ext {
            Option::None
        } else {
            Option::Some(Sync::Ext)
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
