use core::fmt;

use defmt::Format;
use fugit::RateExtU32;
use hash32::{Hash, Hasher};
use heapless::FnvIndexMap;

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

const RATES: [Rate; 17] = [
    Rate::Div(64),
    Rate::Div(32),
    Rate::Div(16),
    Rate::Div(8),
    Rate::Div(5),
    Rate::Div(4),
    Rate::Div(3),
    Rate::Div(2),
    Rate::Unity,
    Rate::Mult(2),
    Rate::Mult(3),
    Rate::Mult(4),
    Rate::Mult(5),
    Rate::Mult(8),
    Rate::Mult(16),
    Rate::Mult(32),
    Rate::Mult(64),
];

pub enum Rate {
    Div(u8),
    Unity,
    Mult(u8),
}

pub enum Pwm {
    P(u8),
    Pew,
}

#[derive(Clone, Copy, Eq, Format, PartialEq)]
pub enum Gate {
    A,
    B,
    C,
    D,
}

impl Hash for Gate {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Gate::A => state.write(&[0]),
            Gate::B => state.write(&[1]),
            Gate::C => state.write(&[2]),
            Gate::D => state.write(&[3]),
        }
    }
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

#[derive(Clone, Copy, Format)]
pub enum GateElement {
    Rate,
    Pwm,
}

#[derive(Clone, Copy, Format)]
pub enum HomeElement {
    Bpm,
    Sync,
}

#[derive(Clone, Copy, Format)]
pub enum Element {
    Gate(Gate, GateElement),
    Home(HomeElement),
}

pub enum StateChange {
    Initialize,
    Bpm(Bpm),
    Sync(Sync),
    PlayStatus(PlayStatus),
    NextPage(Element),
    NextElement(Element),
    None,
}

pub struct GateState {
    rate: Rate,
    pwm: Pwm,
}

impl GateState {
    fn new() -> Self {
        GateState {
            rate: Rate::Unity,
            pwm: Pwm::P(50),
        }
    }
}

pub struct State {
    pub bpm: Bpm,
    sync: Sync,
    play_status: PlayStatus,
    current: Element,
    gates: FnvIndexMap<Gate, GateState, 4>,
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    pub fn new() -> Self {
        let mut gates = FnvIndexMap::<_, _, 4>::new();
        gates.insert(Gate::A, GateState::new()).ok();
        gates.insert(Gate::B, GateState::new()).ok();
        gates.insert(Gate::C, GateState::new()).ok();
        gates.insert(Gate::D, GateState::new()).ok();

        Self {
            bpm: Bpm(120),
            sync: Sync::Ext,
            play_status: PlayStatus::Playing,
            current: Element::Home(HomeElement::Bpm),
            gates,
        }
    }

    pub fn handle_command(&mut self, command: Command) -> StateChange {
        match command {
            Command::EncoderRight => match self.current {
                Element::Home(HomeElement::Bpm) => match self.bpm.next() {
                    Result::Ok(bpm) => StateChange::Bpm(bpm),
                    Result::Err(_) => StateChange::None,
                },
                Element::Home(HomeElement::Sync) => match self.sync.next() {
                    Result::Ok(sync) => StateChange::Sync(sync),
                    Result::Err(_) => StateChange::None,
                },
                _ => todo!(),
            },
            Command::EncoderLeft => match self.current {
                Element::Home(HomeElement::Bpm) => match self.bpm.prev() {
                    Result::Ok(bpm) => StateChange::Bpm(bpm),
                    Result::Err(_) => StateChange::None,
                },
                Element::Home(HomeElement::Sync) => match self.sync.prev() {
                    Result::Ok(sync) => StateChange::Sync(sync),
                    Result::Err(_) => StateChange::None,
                },
                _ => todo!(),
            },
            Command::EncoderPress => StateChange::NextElement(self.next_element()),
            Command::PagePress => StateChange::NextPage(self.next_page()),
            Command::PlayPress => self.toggle_play(),
        }
    }

    fn next_page(&mut self) -> Element {
        self.current = match self.current {
            Element::Home(_) => Element::Gate(Gate::A, GateElement::Rate),
            Element::Gate(Gate::A, _) => Element::Gate(Gate::B, GateElement::Rate),
            Element::Gate(Gate::B, _) => Element::Gate(Gate::C, GateElement::Rate),
            Element::Gate(Gate::C, _) => Element::Gate(Gate::D, GateElement::Rate),
            Element::Gate(Gate::D, _) => Element::Home(HomeElement::Bpm),
        };

        self.current
    }

    fn next_element(&mut self) -> Element {
        self.current = match self.current {
            Element::Home(HomeElement::Bpm) => Element::Home(HomeElement::Sync),
            Element::Home(HomeElement::Sync) => Element::Home(HomeElement::Bpm),
            Element::Gate(gate, GateElement::Rate) => Element::Gate(gate, GateElement::Pwm),
            Element::Gate(gate, GateElement::Pwm) => Element::Gate(gate, GateElement::Rate),
        };

        self.current
    }

    fn toggle_play(&mut self) -> StateChange {
        self.play_status = match self.play_status {
            PlayStatus::Playing => PlayStatus::Paused,
            PlayStatus::Paused => PlayStatus::Playing,
        };

        StateChange::PlayStatus(self.play_status)
    }
}

trait Updatable {
    fn next(&mut self) -> Result<Self, ()>
    where
        Self: Sized;
    fn prev(&mut self) -> Result<Self, ()>
    where
        Self: Sized;
}

#[derive(Clone, Copy, PartialEq, Format)]
pub struct Bpm(pub u32);

impl Bpm {
    pub fn tick_duration(&self) -> MicroSeconds {
        (self.0 / SECONDS_IN_MINUTES * PWM_PERCENT_INCREMENTS * MAX_MULT)
            .Hz::<1, 1>()
            .into_duration()
            .into()
    }
}

impl fmt::Display for Bpm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Updatable for Bpm {
    fn next(&mut self) -> Result<Self, ()> {
        if self.0 == 300 {
            Result::Err(())
        } else {
            self.0 += 1;
            Result::Ok(*self)
        }
    }

    fn prev(&mut self) -> Result<Self, ()> {
        if self.0 == 1 {
            Result::Err(())
        } else {
            self.0 -= 1;
            Result::Ok(*self)
        }
    }
}

#[derive(Clone, Copy, PartialEq, Format)]
pub enum Sync {
    Int,
    Ext,
}

impl Updatable for Sync {
    fn next(&mut self) -> Result<Self, ()> {
        if *self == Sync::Int {
            Result::Err(())
        } else {
            *self = Sync::Int;
            Result::Ok(*self)
        }
    }

    fn prev(&mut self) -> Result<Self, ()> {
        if *self == Sync::Ext {
            Result::Err(())
        } else {
            *self = Sync::Ext;
            Result::Ok(*self)
        }
    }
}

impl fmt::Display for Sync {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int => write!(f, "Int"),
            Self::Ext => write!(f, "Ext"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Format)]
pub enum PlayStatus {
    Playing,
    Paused,
}
