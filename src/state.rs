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

#[derive(Clone, Copy, Format)]
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

#[derive(Clone, Copy, Format)]
pub enum GateElement {
    Div,
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
    Bpm(u32),
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
    gate_a: GateState,
    gate_b: GateState,
    gate_c: GateState,
    gate_d: GateState,
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
            gate_a: GateState::new(),
            gate_b: GateState::new(),
            gate_c: GateState::new(),
            gate_d: GateState::new(),
        }
    }

    pub fn handle_command(&mut self, command: Command) -> StateChange {
        match command {
            Command::EncoderRight => match self.current {
                Element::Home(HomeElement::Bpm) => self.bpm.next(),
                Element::Home(HomeElement::Sync) => self.sync.next(),
                _ => todo!(),
            },
            Command::EncoderLeft => match self.current {
                Element::Home(HomeElement::Bpm) => self.bpm.prev(),
                Element::Home(HomeElement::Sync) => self.sync.prev(),
                _ => todo!(),
            },
            Command::EncoderPress => StateChange::NextElement(self.next_element()),
            Command::PagePress => StateChange::NextPage(self.next_page()),
            Command::PlayPress => self.toggle_play(),
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

    fn toggle_play(&mut self) -> StateChange {
        self.play_status = match self.play_status {
            PlayStatus::Playing => PlayStatus::Paused,
            PlayStatus::Paused => PlayStatus::Playing,
        };

        StateChange::PlayStatus(self.play_status)
    }
}

trait Updatable {
    fn next(&mut self) -> StateChange;
    fn prev(&mut self) -> StateChange;
}

#[derive(PartialEq, Format)]
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

#[derive(Clone, Copy, PartialEq, Format)]
pub enum Sync {
    Int,
    Ext,
}

impl Updatable for Sync {
    fn next(&mut self) -> StateChange {
        if *self == Sync::Int {
            StateChange::None
        } else {
            *self = Sync::Int;
            StateChange::Sync(*self)
        }
    }

    fn prev(&mut self) -> StateChange {
        if *self == Sync::Ext {
            StateChange::None
        } else {
            *self = Sync::Ext;
            StateChange::Sync(*self)
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
