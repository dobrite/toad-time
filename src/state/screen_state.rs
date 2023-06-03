use super::*;

#[derive(Clone)]
pub enum ScreenState {
    Home(Bpm, Sync, PlayStatus),
    Output(Output, OutputConfig, Option<usize>),
}
