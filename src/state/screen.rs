use seq::OutputType;

use super::Output;

#[derive(Clone, Copy)]
pub enum Screen {
    Home,
    Output(Output, OutputType),
}
