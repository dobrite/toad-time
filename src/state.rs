use core::ops::{AddAssign, Deref, DerefMut, SubAssign};
use defmt::Format;
use fugit::RateExtU32;

pub const COMMAND_CAPACITY: usize = 4;
pub const STATE_CHANGE_CAPACITY: usize = 4;
pub const MAX_MULT: u32 = 192;
pub const PWM_PERCENT_INCREMENTS: u32 = 10;
const SECONDS_IN_MINUTES: u32 = 60;

const MICRO_SECONDS_PER_SECOND: u32 = 1_000_000;
pub type MicroSeconds = fugit::Duration<u64, 1, MICRO_SECONDS_PER_SECOND>;

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
    pub bpm: Bpm,
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
            Command::EncoderRight => match forwards(&mut self.bpm) {
                Updated::Yep => StateChange::Bpm(*self.bpm),
                Updated::Nope => StateChange::None,
            },
            Command::EncoderLeft => match backwards(&mut self.bpm) {
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

fn forwards<U: Updatable>(state: &mut U) -> Updated
where
    U: DerefMut<Target = u32> + AddAssign + PartialEq,
{
    if **state == U::MAX {
        Updated::Nope
    } else {
        **state += 1;
        Updated::Yep
    }
}

fn backwards<U: Updatable>(state: &mut U) -> Updated
where
    U: DerefMut<Target = u32> + SubAssign + PartialEq,
{
    if **state == U::MIN {
        Updated::Nope
    } else {
        **state -= 1;
        Updated::Yep
    }
}

trait Updatable {
    const MAX: u32;
    const MIN: u32;
}

#[derive(PartialEq, Format)]
pub struct Bpm(u32);

impl Bpm {
    pub fn tick_duration(&self) -> MicroSeconds {
        (self.0 / SECONDS_IN_MINUTES * PWM_PERCENT_INCREMENTS * MAX_MULT)
            .Hz::<1, 1>()
            .into_duration()
            .into()
    }
}

impl Deref for Bpm {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Bpm {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Updatable for Bpm {
    const MAX: u32 = 300;
    const MIN: u32 = 1;
}

impl AddAssign for Bpm {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

impl SubAssign for Bpm {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
    }
}
