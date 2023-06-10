use core::fmt;

use defmt::Format;

use super::Updatable;

pub const MIN_BPM: u32 = 1;
pub const MAX_BPM: u32 = 300;

#[derive(Clone, Copy, Format)]
pub struct Bpm(pub u32);

impl fmt::Display for Bpm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Updatable for Bpm {
    fn next(&self) -> Option<Self> {
        if self.0 == MAX_BPM {
            Option::None
        } else {
            Option::Some(Self(self.0 + 1))
        }
    }

    fn prev(&self) -> Option<Self> {
        if self.0 == MIN_BPM {
            Option::None
        } else {
            Option::Some(Self(self.0 - 1))
        }
    }
}
