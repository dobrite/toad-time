use heapless::FnvIndexMap;
use seq::{Prob, Pwm, Rate};

use self::home::Home;
pub use self::{
    bpm::Bpm, command::Command, element::Element, gate_state::GateState, output::Output,
    play_status::PlayStatus, prob::ProbString, rate::RateString, state::State,
    state_change::StateChange, sync::Sync,
};

mod bpm;
mod command;
mod element;
mod gate_state;
mod home;
mod output;
mod play_status;
mod prob;
mod pwm;
mod rate;
#[allow(clippy::module_inception)]
mod state;
mod state_change;
mod sync;

pub type Outputs = FnvIndexMap<Output, GateState, 4>;

trait Updatable {
    fn next(&self) -> Option<Self>
    where
        Self: Sized;
    fn prev(&self) -> Option<Self>
    where
        Self: Sized;
}
