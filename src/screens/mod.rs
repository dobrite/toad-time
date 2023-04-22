use crate::{display::Display, state::StateChange};

mod home;

pub use home::Home;

enum Screen {
    Home,
    GateA,
    GateB,
    GateC,
    GateD,
}

enum HomeElement {
    Bpm,
    //Sync,
}

//enum GateElement {
//    Div,
//    Pwm,
//}

enum Element {
    Home(HomeElement),
    //Gate(GateElement),
}

pub struct Screens {
    home: Home,
    current_screen: Screen,
    current_element: Element,
}

impl Screens {
    pub fn new(home: Home) -> Self {
        Self {
            home,
            current_screen: Screen::Home,
            current_element: Element::Home(HomeElement::Bpm),
        }
    }

    pub fn handle_state_change(&mut self, state_change: StateChange, display: &mut Display) {
        match state_change {
            StateChange::Bpm(bpm) => match self.current_screen {
                Screen::Home => self.home.draw_bpm(display, bpm),
                Screen::GateA | Screen::GateB | Screen::GateC | Screen::GateD => {}
            },
            StateChange::None => unreachable!(),
        }
    }

    pub fn draw(&mut self, display: &mut Display) {
        display.clear();
        self.home.draw(display);
        display.flush();
    }
}
