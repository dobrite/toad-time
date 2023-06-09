use seq::OutputType;

use super::Output;

#[derive(Clone, Copy)]
pub enum Screen {
    Home,
    Output(Output, OutputType),
}

impl Screen {
    pub fn is_euclid(&self, current_output: Output) -> bool {
        if let Screen::Output(output, output_type) = self {
            if current_output == *output {
                *output_type == OutputType::Euclid
            } else {
                false
            }
        } else {
            false
        }
    }
}
