use seq::Density;

use super::Updatable;

impl Updatable for Density {
    fn next(&self) -> Option<Self> {
        if self.0 == 16 {
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
