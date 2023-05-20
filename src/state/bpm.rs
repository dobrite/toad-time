use core::fmt;

use defmt::Format;

use super::Updatable;

#[derive(Clone, Copy, Format)]
pub struct Bpm(pub u32);

impl fmt::Display for Bpm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Updatable for Bpm {
    fn next(&self) -> Option<Self> {
        if self.0 == 300 {
            Option::None
        } else {
            Option::Some(Self(self.0 + 1))
        }
    }

    fn prev(&self) -> Option<Self> {
        if self.0 == 1 {
            Option::None
        } else {
            Option::Some(Self(self.0 - 1))
        }
    }
}