use core::ops::Deref;
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

enum Updated {
    Yep,
    Nope,
}

pub enum StateChange {
    Bpm(u32),
    None,
}

pub struct State {
    bpm: Bpm,
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    pub fn new() -> Self {
        Self { bpm: Bpm(120) }
    }

    pub fn bpm(&self) -> u32 {
        *self.bpm
    }

    pub fn handle_command(&mut self, command: Command) -> StateChange {
        match command {
            Command::EncoderRight => match self.bpm.forwards() {
                Updated::Yep => StateChange::Bpm(*self.bpm),
                Updated::Nope => StateChange::None,
            },
            Command::EncoderLeft => match self.bpm.backwards() {
                Updated::Yep => StateChange::Bpm(*self.bpm),
                Updated::Nope => StateChange::None,
            },
            //Command::EncoderPress => {}
            //Command::PagePress => {}
            //Command::PlayPress => {}
            _ => unreachable!(),
        }
    }
}

trait Updatable {
    const MAX: u32;
    const MIN: u32;

    fn forwards(&mut self) -> Updated;
    fn backwards(&mut self) -> Updated;
}

#[derive(Format)]
struct Bpm(u32);

impl Deref for Bpm {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Updatable for Bpm {
    const MAX: u32 = 300;
    const MIN: u32 = 1;

    fn forwards(&mut self) -> Updated {
        if self.0 == Self::MAX {
            Updated::Nope
        } else {
            self.0 += 1;
            Updated::Yep
        }
    }

    fn backwards(&mut self) -> Updated {
        if self.0 == Self::MIN {
            Updated::Nope
        } else {
            self.0 -= 1;
            Updated::Yep
        }
    }
}
