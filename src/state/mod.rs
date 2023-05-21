use heapless::FnvIndexMap;
use seq::{LaneConfig as OutputConfig, Prob, Pwm, Rate};

use self::home::Home;
pub use self::{
    bpm::Bpm, command::Command, element::Element, output::Output, play_status::PlayStatus,
    prob::ProbString, rate::RateString, state::State, state_change::StateChange, sync::Sync,
};

mod bpm;
mod command;
mod element;
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

trait Updatable {
    fn next(&self) -> Option<Self>
    where
        Self: Sized;
    fn prev(&self) -> Option<Self>
    where
        Self: Sized;
}
