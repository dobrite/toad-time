use super::{state::StateChange, ScreenState};

pub struct StateMemo {
    pub current_screen: ScreenState,
}

impl StateMemo {
    pub fn new(current_screen: ScreenState) -> Self {
        Self { current_screen }
    }

    pub fn update(&mut self, state_change: &StateChange) {
        match state_change {
            StateChange::NextScreen(ref next_screen) => self.current_screen = next_screen.clone(),
            StateChange::OutputType(ref screen_state) => {
                self.current_screen = screen_state.clone();
            }
            _ => {}
        }
    }
}
