use heapless::String;
use seq::OutputType;

use super::Updatable;

impl Updatable for OutputType {
    fn next(&mut self) -> Option<Self> {
        match self {
            OutputType::Euclid => Option::None,
            OutputType::Gate => {
                *self = OutputType::Euclid;
                Option::Some(*self)
            }
        }
    }

    fn prev(&mut self) -> Option<Self> {
        match self {
            OutputType::Gate => Option::None,
            OutputType::Euclid => {
                *self = OutputType::Gate;
                Option::Some(*self)
            }
        }
    }
}

pub struct OutputTypeString(pub String<3>);

impl From<&OutputType> for OutputTypeString {
    fn from(val: &OutputType) -> Self {
        let output_type_string = match val {
            OutputType::Gate => "G",
            OutputType::Euclid => "E",
        };

        OutputTypeString(output_type_string.into())
    }
}
