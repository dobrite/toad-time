use seq::Density;

use super::Updatable;

impl Updatable for Density {
    fn next(&mut self) -> Option<Self> {
        if self.0 == 16 {
            Option::None
        } else {
            self.0 += 1;
            Option::Some(*self)
        }
    }

    fn prev(&mut self) -> Option<Self> {
        if self.0 == 1 {
            Option::None
        } else {
            self.0 -= 1;
            Option::Some(*self)
        }
    }
}
