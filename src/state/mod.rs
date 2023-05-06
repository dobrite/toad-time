use defmt::Format;
use heapless::FnvIndexMap;
use seq::{Prob, Pwm, Rate};

pub use self::{
    bpm::Bpm,
    command::Command,
    element::{Element, Gate, Home},
    gate_state::GateState,
    prob::ProbString,
    rate::RateString,
    state::State,
    state_change::StateChange,
    sync::Sync,
};

mod bpm;
mod command;
mod element;
mod gate_state;
mod prob;
mod pwm;
mod rate;
#[allow(clippy::module_inception)]
mod state;
mod state_change;
mod sync;

pub const COMMAND_CAPACITY: usize = 4;
pub const STATE_CHANGE_CAPACITY: usize = 4;
pub const MAX_MULT: u32 = 192;
pub const PWM_PERCENT_INCREMENTS: u32 = 10;
const SECONDS_IN_MINUTES: f32 = 60.0;
const MICRO_SECONDS_PER_SECOND: u32 = 1_000_000;

pub type MicroSeconds = fugit::Duration<u64, 1, MICRO_SECONDS_PER_SECOND>;
pub type Gates = FnvIndexMap<Gate, GateState, 4>;

trait Updatable {
    fn next(&mut self) -> Option<Self>
    where
        Self: Sized;
    fn prev(&mut self) -> Option<Self>
    where
        Self: Sized;
}

#[derive(Clone, Copy, PartialEq, Format)]
pub enum PlayStatus {
    Playing,
    Paused,
}
