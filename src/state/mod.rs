use heapless::FnvIndexMap;
use seq::OutputConfig;

use self::home::Home;
pub use self::{
    bpm::Bpm, command::Command, element::Element, output::Output, play_status::PlayStatus,
    prob::ProbString, rate::RateString, screen::Screen, state::State, state_change::StateChange,
    sync::Sync,
};

mod bpm;
mod command;
mod element;
mod home;
mod output;
mod output_type;
mod play_status;
mod prob;
mod pwm;
mod rate;
mod screen;
#[allow(clippy::module_inception)]
mod state;
mod state_change;
mod sync;

pub type Outputs = FnvIndexMap<Output, OutputConfig, 4>;

trait Updatable {
    fn next(&self) -> Option<Self>
    where
        Self: Sized;
    fn prev(&self) -> Option<Self>
    where
        Self: Sized;
}
