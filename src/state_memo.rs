use super::ScreenState;

pub struct StateMemo {
    pub current_screen: ScreenState,
}

impl StateMemo {
    pub fn new(current_screen: ScreenState) -> Self {
        Self { current_screen }
    }
}
