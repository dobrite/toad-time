use core::fmt;

use defmt::Format;

use super::Updatable;

#[derive(Clone, Copy, Format)]
pub enum Sync {
    Int,
    Ext,
}

impl Updatable for Sync {
    fn next(&mut self) -> Option<Self> {
        match self {
            Sync::Int => Option::None,
            Sync::Ext => {
                *self = Sync::Int;
                Option::Some(*self)
            }
        }
    }

    fn prev(&mut self) -> Option<Self> {
        match self {
            Sync::Ext => Option::None,
            Sync::Int => {
                *self = Sync::Ext;
                Option::Some(*self)
            }
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
