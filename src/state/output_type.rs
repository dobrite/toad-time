use heapless::String;
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
