use core::fmt;

use defmt::Format;
use fugit::RateExtU32;
use hash32::{Hash, Hasher};
use heapless::{FnvIndexMap, String};
use seq::{Pwm, Rate};

pub const COMMAND_CAPACITY: usize = 4;
pub const STATE_CHANGE_CAPACITY: usize = 4;
pub const MAX_MULT: u32 = 192;
pub const PWM_PERCENT_INCREMENTS: u32 = 10;
const SECONDS_IN_MINUTES: f32 = 60.0;

const MICRO_SECONDS_PER_SECOND: u32 = 1_000_000;
pub type MicroSeconds = fugit::Duration<u64, 1, MICRO_SECONDS_PER_SECOND>;

pub type Gates = FnvIndexMap<Gate, GateState, 4>;

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

impl Updatable for Rate {
    fn next(&mut self) -> Option<Self> {
        if self == RATES.last().unwrap() {
            Option::None
        } else {
            let index = RATES.iter().position(|r| r == self).unwrap() + 1;
            *self = RATES[index];
            Option::Some(*self)
        }
    }

    fn prev(&mut self) -> Option<Self> {
        if self == RATES.first().unwrap() {
            Option::None
        } else {
            let index = RATES.iter().position(|r| r == self).unwrap() - 1;
            *self = RATES[index];
            Option::Some(*self)
        }
    }
}

pub struct RateString(pub String<3>);

impl From<Rate> for RateString {
    fn from(val: Rate) -> Self {
        let rate_string = match val {
            Rate::Div(64) => "/64",
            Rate::Div(32) => "/32",
            Rate::Div(16) => "/16",
            Rate::Div(8) => "/8",
            Rate::Div(5) => "/5",
            Rate::Div(4) => "/4",
            Rate::Div(3) => "/3",
            Rate::Div(2) => "/2",
            Rate::Unity => "x1",
            Rate::Mult(2) => "x2",
            Rate::Mult(3) => "x3",
            Rate::Mult(4) => "x4",
            Rate::Mult(5) => "x5",
            Rate::Mult(8) => "x8",
            Rate::Mult(16) => "x16",
            Rate::Mult(32) => "x32",
            Rate::Mult(64) => "x64",
            _ => unreachable!(),
        };

        RateString(rate_string.into())
    }
}

impl Updatable for Pwm {
    fn next(&mut self) -> Option<Self> {
        let next = match self {
            Pwm::P90 => Pwm::P90,
            Pwm::P80 => Pwm::P90,
            Pwm::P70 => Pwm::P80,
            Pwm::P60 => Pwm::P70,
            Pwm::P50 => Pwm::P60,
            Pwm::P40 => Pwm::P50,
            Pwm::P30 => Pwm::P40,
            Pwm::P20 => Pwm::P30,
            Pwm::P10 => Pwm::P20,
            Pwm::Pew => Pwm::P10,
        };

        let output = match self {
            Pwm::P90 => Option::None,
            Pwm::P80 => Option::Some(Pwm::P90),
            Pwm::P70 => Option::Some(Pwm::P80),
            Pwm::P60 => Option::Some(Pwm::P70),
            Pwm::P50 => Option::Some(Pwm::P60),
            Pwm::P40 => Option::Some(Pwm::P50),
            Pwm::P30 => Option::Some(Pwm::P40),
            Pwm::P20 => Option::Some(Pwm::P30),
            Pwm::P10 => Option::Some(Pwm::P20),
            Pwm::Pew => Option::Some(Pwm::P10),
        };

        *self = next;

        output
    }

    fn prev(&mut self) -> Option<Self> {
        let prev = match self {
            Pwm::Pew => Pwm::Pew,
            Pwm::P10 => Pwm::Pew,
            Pwm::P20 => Pwm::P10,
            Pwm::P30 => Pwm::P20,
            Pwm::P40 => Pwm::P30,
            Pwm::P50 => Pwm::P40,
            Pwm::P60 => Pwm::P50,
            Pwm::P70 => Pwm::P60,
            Pwm::P80 => Pwm::P70,
            Pwm::P90 => Pwm::P80,
        };

        let output = match self {
            Pwm::Pew => Option::None,
            Pwm::P10 => Option::Some(Pwm::Pew),
            Pwm::P20 => Option::Some(Pwm::P10),
            Pwm::P30 => Option::Some(Pwm::P20),
            Pwm::P40 => Option::Some(Pwm::P30),
            Pwm::P50 => Option::Some(Pwm::P40),
            Pwm::P60 => Option::Some(Pwm::P50),
            Pwm::P70 => Option::Some(Pwm::P60),
            Pwm::P80 => Option::Some(Pwm::P70),
            Pwm::P90 => Option::Some(Pwm::P80),
        };

        *self = prev;

        output
    }
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

#[derive(Clone, Copy)]
pub struct Home;

#[derive(Clone, Copy)]
pub enum Element {
    Rate(Gate),
    Pwm(Gate),
    Bpm(Home),
    Sync(Home),
}

impl Element {
    fn next(&self, state: &mut State) -> StateChange {
        match self {
            Element::Bpm(Home) => state.bpm.next().into(),
            Element::Sync(Home) => state.sync.next().into(),
            Element::Rate(gate) => match state.gates[gate].rate.next() {
                Option::Some(rate) => StateChange::Rate(*gate, rate),
                Option::None => StateChange::None,
            },
            Element::Pwm(gate) => match state.gates[gate].pwm.next() {
                Option::Some(pwm) => StateChange::Pwm(*gate, pwm),
                Option::None => StateChange::None,
            },
        }
    }

    fn prev(&self, state: &mut State) -> StateChange {
        match self {
            Element::Bpm(Home) => state.bpm.prev().into(),
            Element::Sync(Home) => state.sync.prev().into(),
            Element::Rate(gate) => match state.gates[gate].rate.prev() {
                Option::Some(rate) => StateChange::Rate(*gate, rate),
                Option::None => StateChange::None,
            },
            Element::Pwm(gate) => match state.gates[gate].pwm.prev() {
                Option::Some(pwm) => StateChange::Pwm(*gate, pwm),
                Option::None => StateChange::None,
            },
        }
    }
}

pub enum StateChange {
    Initialize,
    Bpm(Bpm),
    Sync(Sync),
    Rate(Gate, Rate),
    Pwm(Gate, Pwm),
    PlayStatus(PlayStatus),
    NextPage(Element),
    NextElement(Element),
    None,
}

impl From<Option<Bpm>> for StateChange {
    fn from(val: Option<Bpm>) -> Self {
        match val {
            Option::Some(bpm) => StateChange::Bpm(bpm),
            Option::None => StateChange::None,
        }
    }
}

impl From<Option<Sync>> for StateChange {
    fn from(val: Option<Sync>) -> Self {
        match val {
            Option::Some(sync) => StateChange::Sync(sync),
            Option::None => StateChange::None,
        }
    }
}

pub struct GateState {
    pub rate: Rate,
    pub pwm: Pwm,
}

impl Default for GateState {
    fn default() -> Self {
        Self::new()
    }
}

impl GateState {
    pub fn new() -> Self {
        GateState {
            rate: Rate::Unity,
            pwm: Pwm::P50,
        }
    }
}

pub struct State {
    pub bpm: Bpm,
    pub sync: Sync,
    pub play_status: PlayStatus,
    pub current: Element,
    pub gates: Gates,
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    pub fn new() -> Self {
        let mut gates = Gates::new();
        gates.insert(Gate::A, GateState::new()).ok();
        gates.insert(Gate::B, GateState::new()).ok();
        gates.insert(Gate::C, GateState::new()).ok();
        gates.insert(Gate::D, GateState::new()).ok();

        Self {
            bpm: Bpm(120),
            sync: Sync::Ext,
            play_status: PlayStatus::Playing,
            current: Element::Bpm(Home),
            gates,
        }
    }

    pub fn gate_configs(&self) -> bool {
        true
    }

    pub fn tick_duration(&self) -> MicroSeconds {
        let bpm: f32 = self.bpm.0 as f32;
        let bps = bpm / SECONDS_IN_MINUTES;
        const MULTIPLYER: f32 = (PWM_PERCENT_INCREMENTS * MAX_MULT) as f32;
        let hertz: u32 = (bps * MULTIPLYER) as u32;

        hertz.Hz::<1, 1>().into_duration().into()
    }

    pub fn handle_command(&mut self, command: Command) -> StateChange {
        let current = self.current;

        match command {
            Command::EncoderRight => current.next(self),
            Command::EncoderLeft => current.prev(self),
            Command::EncoderPress => StateChange::NextElement(self.next_element()),
            Command::PagePress => StateChange::NextPage(self.next_page()),
            Command::PlayPress => self.toggle_play(),
        }
    }

    fn next_page(&mut self) -> Element {
        self.current = match self.current {
            Element::Bpm(_) => Element::Rate(Gate::A),
            Element::Sync(_) => Element::Rate(Gate::A),
            Element::Rate(Gate::A) => Element::Rate(Gate::B),
            Element::Pwm(Gate::A) => Element::Rate(Gate::B),
            Element::Rate(Gate::B) => Element::Rate(Gate::C),
            Element::Pwm(Gate::B) => Element::Rate(Gate::C),
            Element::Rate(Gate::C) => Element::Rate(Gate::D),
            Element::Pwm(Gate::C) => Element::Rate(Gate::D),
            Element::Rate(Gate::D) => Element::Bpm(Home),
            Element::Pwm(Gate::D) => Element::Bpm(Home),
        };

        self.current
    }

    fn next_element(&mut self) -> Element {
        self.current = match self.current {
            Element::Bpm(_) => Element::Sync(Home),
            Element::Sync(_) => Element::Bpm(Home),
            Element::Rate(gate) => Element::Pwm(gate),
            Element::Pwm(gate) => Element::Rate(gate),
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
    fn next(&mut self) -> Option<Self>
    where
        Self: Sized;
    fn prev(&mut self) -> Option<Self>
    where
        Self: Sized;
}

#[derive(Clone, Copy, PartialEq, Format)]
pub struct Bpm(pub u32);

impl fmt::Display for Bpm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Updatable for Bpm {
    fn next(&mut self) -> Option<Self> {
        if self.0 == 300 {
            Option::None
        } else {
            self.0 += 1;
            Option::Some(*self)
        }
    }

    fn prev(&mut self) -> Option<Self> {
        if self.0 == 1 {
            Option::None
        } else {
            self.0 -= 1;
            Option::Some(*self)
        }
    }
}

#[derive(Clone, Copy, PartialEq, Format)]
pub enum Sync {
    Int,
    Ext,
}

impl Updatable for Sync {
    fn next(&mut self) -> Option<Self> {
        if *self == Sync::Int {
            Option::None
        } else {
            *self = Sync::Int;
            Option::Some(*self)
        }
    }

    fn prev(&mut self) -> Option<Self> {
        if *self == Sync::Ext {
            Option::None
        } else {
            *self = Sync::Ext;
            Option::Some(*self)
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
