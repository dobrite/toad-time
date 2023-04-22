use crate::{
    display::Display,
    state::{StateChange, Sync},
};

mod home;

pub use home::Home;

pub struct Screens {
    home: Home,
    state: ScreenState,
}

pub struct ScreenState {
    bpm: u32,
    sync: Sync,
    is_playing: bool,
}

impl Default for ScreenState {
    fn default() -> Self {
        Self::new()
    }
}

impl ScreenState {
    pub fn new() -> Self {
        Self {
            bpm: 120,
            sync: Sync::Ext,
            is_playing: true,
        }
    }
}

impl Screens {
    pub fn new(home: Home) -> Self {
        Self {
            home,
            state: Default::default(),
        }
    }

    pub fn handle_state_change(&mut self, state_change: StateChange) {
        match state_change {
            StateChange::Bpm(bpm) => self.state.bpm = bpm,
            StateChange::None => unreachable!(),
        }
    }

    pub fn draw(&mut self, display: &mut Display) {
        display.clear();
        self.home.draw(&self.state, display);
        display.flush();
    }
}
