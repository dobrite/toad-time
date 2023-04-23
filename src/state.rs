use core::{
    fmt,
    ops::{AddAssign, Deref, DerefMut, SubAssign},
};

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

#[derive(Clone, Copy)]
pub enum HomeElement {
    Bpm,
    //Sync,
}

#[derive(Clone, Copy)]
pub enum GateElement {
    Div,
    // Pwm,
}

#[derive(Clone, Copy)]
pub enum Element {
    Home(HomeElement),
    GateA(GateElement),
    GateB(GateElement),
    GateC(GateElement),
    GateD(GateElement),
}

pub enum StateChange {
    Initialize,
    Bpm(u32),
    NextPage(Element),
    None,
}

pub struct State {
    pub bpm: Bpm,
    #[allow(dead_code)]
    sync: Sync,
    #[allow(dead_code)]
    play_status: PlayStatus,
    current: Element,
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    pub fn new() -> Self {
        Self {
            bpm: Bpm(120),
            sync: Sync::Ext,
            play_status: PlayStatus::Playing,
            current: Element::Home(HomeElement::Bpm),
        }
    }

    pub fn handle_command(&mut self, command: Command) -> StateChange {
        match command {
            Command::EncoderRight => match self.current {
                Element::Home(HomeElement::Bpm) => match forwards(&mut self.bpm) {
                    Updated::Yep => StateChange::Bpm(*self.bpm),
                    Updated::Nope => StateChange::None,
                },
                _ => todo!(),
            },
            Command::EncoderLeft => match self.current {
                Element::Home(HomeElement::Bpm) => match backwards(&mut self.bpm) {
                    Updated::Yep => StateChange::Bpm(*self.bpm),
                    Updated::Nope => StateChange::None,
                },
                _ => todo!(),
            },
            Command::EncoderPress => StateChange::None,
            Command::PagePress => StateChange::NextPage(self.next_page()),
            Command::PlayPress => StateChange::None,
        }
    }

    fn next_page(&mut self) -> Element {
        self.current = match self.current {
            Element::Home(_) => Element::GateA(GateElement::Div),
            Element::GateA(_) => Element::GateB(GateElement::Div),
            Element::GateB(_) => Element::GateC(GateElement::Div),
            Element::GateC(_) => Element::GateD(GateElement::Div),
            Element::GateD(_) => Element::Home(HomeElement::Bpm),
        };

        self.current
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

pub enum Sync {
    #[allow(dead_code)]
    Int,
    Ext,
}

impl fmt::Display for Sync {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Sync::Int => write!(f, "Int"),
            Sync::Ext => write!(f, "Ext"),
        }
    }
}

#[derive(PartialEq)]
pub enum PlayStatus {
    Playing,
    #[allow(dead_code)]
    Paused,
}
