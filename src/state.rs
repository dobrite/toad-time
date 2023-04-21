use defmt::Format;

pub const COMMAND_CAPACITY: usize = 4;
pub const STATE_CHANGE_CAPACITY: usize = 4;

#[derive(Clone, Copy, Format)]
pub enum Command {
    EncoderRight,
    EncoderLeft,
    EncoderPress,
    PagePress,
    PlayPress,
}

pub enum StateChange {
    Bpm(u32),
}

pub struct State {
    pub bpm: u32,
}

impl State {
    pub fn new() -> Self {
        Self { bpm: 120 }
    }
}
