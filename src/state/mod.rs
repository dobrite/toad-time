use heapless::FnvIndexMap;
use seq::{Prob, Pwm, Rate};

use self::element::Home;
pub use self::{
    bpm::Bpm,
    command::Command,
    element::{Element, Gate},
    gate_state::GateState,
    play_status::PlayStatus,
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
mod play_status;
mod prob;
mod pwm;
mod rate;
#[allow(clippy::module_inception)]
mod state;
mod state_change;
mod sync;

pub const MAX_MULT: u32 = 192;
pub const PWM_PERCENT_INCREMENTS: u32 = 10;
const SECONDS_IN_MINUTES: f32 = 60.0;

pub type Gates = FnvIndexMap<Gate, GateState, 4>;

trait Updatable {
    fn next(&mut self) -> Option<Self>
    where
        Self: Sized;
    fn prev(&mut self) -> Option<Self>
    where
        Self: Sized;
}
