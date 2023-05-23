use seq::OutputType;

use super::Updatable;

impl Updatable for OutputType {
    fn next(&self) -> Option<Self> {
        match self {
            OutputType::Euclid => Option::None,
            OutputType::Gate => Option::Some(OutputType::Euclid),
        }
    }

    fn prev(&self) -> Option<Self> {
        match self {
            OutputType::Gate => Option::None,
            OutputType::Euclid => Option::Some(OutputType::Gate),
        }
    }
}
