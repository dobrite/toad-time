use seq::OutputConfig;

pub use self::{
    bpm::Bpm,
    bpm_sync::BpmSync,
    command::Command,
    element::Element,
    output::Output,
    output_type::OutputTypeString,
    play_status::PlayStatus,
    prob::ProbString,
    rate::RateString,
    screen::Screen,
    screen_state::{HomeScreenState, OutputScreenState, ScreenState},
    sequence_state::SequenceState,
    state::State,
    state_change::StateChange,
    sync::Sync,
};

mod bpm;
mod bpm_sync;
mod command;
mod density;
mod element;
mod length;
mod output;
mod output_type;
mod play_status;
mod prob;
mod pwm;
mod rate;
mod screen;
mod screen_state;
mod sequence_state;
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
