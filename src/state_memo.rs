use super::{state::StateChange, Screen};

pub struct StateMemo {
    pub current_screen: Screen,
}

impl StateMemo {
    pub fn new(current_screen: Screen) -> Self {
        Self { current_screen }
    }

    pub fn update(&mut self, state_change: &StateChange) {
        match state_change {
            StateChange::NextScreen(next_screen) => self.current_screen = next_screen.into(),
            StateChange::OutputType(screen_state) => self.current_screen = screen_state.into(),
            _ => {}
        }
    }
}
