use core::fmt;

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

#[derive(Clone, Copy)]
pub enum Gate {
    A,
    B,
    C,
    D,
}

impl fmt::Display for Gate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::A => write!(f, "A"),
            Self::B => write!(f, "B"),
            Self::C => write!(f, "C"),
            Self::D => write!(f, "D"),
        }
    }
}

#[derive(Clone, Copy)]
pub enum GateElement {
    Div,
    Pwm,
}

#[derive(Clone, Copy)]
pub enum HomeElement {
    Bpm,
    Sync,
}

#[derive(Clone, Copy)]
pub enum Element {
    Gate(Gate, GateElement),
    Home(HomeElement),
}

pub enum StateChange {
    Initialize,
    Bpm(u32),
    NextPage(Element),
    NextElement(Element),
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
                Element::Home(HomeElement::Bpm) => self.bpm.next(),
                _ => todo!(),
            },
            Command::EncoderLeft => match self.current {
                Element::Home(HomeElement::Bpm) => self.bpm.prev(),
                _ => todo!(),
            },
            Command::EncoderPress => StateChange::NextElement(self.next_element()),
            Command::PagePress => StateChange::NextPage(self.next_page()),
            Command::PlayPress => StateChange::None,
        }
    }

    fn next_page(&mut self) -> Element {
        self.current = match self.current {
            Element::Home(_) => Element::Gate(Gate::A, GateElement::Div),
            Element::Gate(Gate::A, _) => Element::Gate(Gate::B, GateElement::Div),
            Element::Gate(Gate::B, _) => Element::Gate(Gate::C, GateElement::Div),
            Element::Gate(Gate::C, _) => Element::Gate(Gate::D, GateElement::Div),
            Element::Gate(Gate::D, _) => Element::Home(HomeElement::Bpm),
        };

        self.current
    }

    fn next_element(&mut self) -> Element {
        self.current = match self.current {
            Element::Home(HomeElement::Bpm) => Element::Home(HomeElement::Sync),
            Element::Home(HomeElement::Sync) => Element::Home(HomeElement::Bpm),
            Element::Gate(gate, GateElement::Div) => Element::Gate(gate, GateElement::Pwm),
            Element::Gate(gate, GateElement::Pwm) => Element::Gate(gate, GateElement::Div),
        };

        self.current
    }
}

trait Updatable {
    fn next(&mut self) -> StateChange;
    fn prev(&mut self) -> StateChange;
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

impl Updatable for Bpm {
    fn next(&mut self) -> StateChange {
        if self.0 == 300 {
            StateChange::None
        } else {
            self.0 += 1;
            StateChange::Bpm(self.0)
        }
    }

    fn prev(&mut self) -> StateChange {
        if self.0 == 1 {
            StateChange::None
        } else {
            self.0 -= 1;
            StateChange::Bpm(self.0)
        }
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
            Self::Int => write!(f, "Int"),
            Self::Ext => write!(f, "Ext"),
        }
    }
}

#[derive(PartialEq)]
pub enum PlayStatus {
    Playing,
    #[allow(dead_code)]
    Paused,
}
