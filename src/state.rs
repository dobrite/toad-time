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
    None,
}

pub struct State {
    pub bpm: u32,
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    pub fn new() -> Self {
        Self { bpm: 120 }
    }

    pub fn handle_command(&mut self, command: Command) -> StateChange {
        match command {
            Command::EncoderRight => {
                // logic to decide what piece of state to change
                self.bpm += 1;
                StateChange::Bpm(self.bpm)
            }
            Command::EncoderLeft => {
                // logic to decide what piece of state to change
                self.bpm -= 1;
                StateChange::Bpm(self.bpm)
            }
            //Command::EncoderPress => {}
            //Command::PagePress => {}
            //Command::PlayPress => {}
            _ => unreachable!(),
        }
    }
}
