use core::fmt;

use defmt::Format;

use super::Updatable;

#[derive(Clone, Copy, Format)]
pub enum Sync {
    Int,
    Ext,
}

impl Updatable for Sync {
    fn next(&self) -> Option<Self> {
        match self {
            Sync::Ext => Option::None,
            Sync::Int => Option::Some(Sync::Ext),
        }
    }

    fn prev(&self) -> Option<Self> {
        match self {
            Sync::Int => Option::None,
            Sync::Ext => Option::Some(Sync::Int),
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
