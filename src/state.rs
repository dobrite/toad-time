use seq::OutputConfig;

pub use self::{
    bpm::Bpm,
    command::Command,
    element::Element,
    output::Output,
    output_type::OutputTypeString,
    play_status::PlayStatus,
    prob::ProbString,
    rate::RateString,
    screen_state::{HomeScreenState, OutputScreenState, ScreenState},
    state::State,
    state_change::StateChange,
    sync::Sync,
};

mod bpm;
mod command;
mod density;
mod element;
mod home;
mod length;
mod output;
mod output_type;
mod play_status;
mod prob;
mod pwm;
mod rate;
mod screen_state;
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
