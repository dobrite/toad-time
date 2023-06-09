use embedded_graphics::prelude::Point;
use heapless::String;

use crate::{
    screens::Display,
    state::{Bpm, Element, HomeScreenState, PlayStatus, ScreenState, Sync},
    StateChange,
};

pub struct HomeScreen {
    bpm_label: String<3>,
    bpm_str: String<3>,
    sync_str: String<3>,
}

impl Default for HomeScreen {
    fn default() -> Self {
        Self::new()
    }
}

impl HomeScreen {
    pub fn new() -> Self {
        let bpm_label = String::new();
        let bpm_str = String::new();
        let sync_str = String::new();

        Self {
            bpm_label,
            bpm_str,
            sync_str,
        }
    }

    pub fn draw(&mut self, state_change: StateChange, display: &mut Display) {
        match state_change {
            StateChange::Bpm(bpm) => {
                self.clear_bpm_value(display);
                self.draw_bpm_value(display, bpm);
            }
            StateChange::Sync(sync) => {
                self.clear_sync(display);
                self.draw_sync(display, sync);
            }
            StateChange::PlayStatus(_, play_status) => self.draw_play_pause(display, play_status),
            StateChange::NextElement(_, previous_element, current_element) => {
                self.clear_pointer(display, previous_element);
                self.draw_pointer(display, current_element);
            }
            StateChange::NextScreen(screen_state) => {
                self.redraw_screen(display, screen_state, Element::Bpm);
            }
            _ => {}
        }
    }

    fn redraw_screen(
        &mut self,
        display: &mut Display,
        screen_state: ScreenState,
        element: Element,
    ) {
        if let ScreenState::Home(HomeScreenState {
            bpm,
            sync,
            play_status,
        }) = screen_state
        {
            display.clear();
            self.draw_bpm_label(display);
            self.draw_bpm_value(display, bpm);
            self.draw_frogge(display);
            self.draw_sync(display, sync);
            self.draw_play_pause(display, play_status);
            self.draw_pointer(display, element);
        }
    }

    fn draw_bpm_label(&mut self, display: &mut Display) {
        display.draw_smol_text(&mut self.bpm_label, "BPM", Point::new(68, 27));
    }

    fn clear_bpm_value(&mut self, display: &mut Display) {
        display.clear_bigge_text(&self.bpm_str, Point::new(22, 30));
    }

    fn draw_bpm_value(&mut self, display: &mut Display, bpm: Bpm) {
        display.draw_bigge_text(&mut self.bpm_str, bpm, Point::new(22, 30));
    }

    fn draw_frogge(&mut self, display: &mut Display) {
        let point = Point::new(80, 26);
        let index = 0;
        display.clear_frogge(point);
        display.draw_frogge(index, point);
    }

    fn clear_sync(&mut self, display: &mut Display) {
        display.clear_smol_text(&self.sync_str, Point::new(22, 50));
    }

    fn draw_sync(&mut self, display: &mut Display, sync: Sync) {
        display.draw_smol_text(&mut self.sync_str, sync, Point::new(22, 50));
    }

    fn draw_play_pause(&mut self, display: &mut Display, play_status: PlayStatus) {
        let point = Point::new(56, 30);
        let index = match play_status {
            PlayStatus::Playing => 0,
            PlayStatus::Paused => 1,
        };
        display.clear_play_pause(point);
        display.draw_play_pause(index, point);
    }

    fn clear_pointer(&mut self, display: &mut Display, element: Element) {
        match element {
            Element::Bpm => display.clear_pointer_right(Point::new(4, 8)),
            Element::Sync => display.clear_pointer_right(Point::new(4, 32)),
            _ => {}
        };
    }

    fn draw_pointer(&mut self, display: &mut Display, element: Element) {
        match element {
            Element::Bpm => display.draw_pointer_right(Point::new(4, 8)),
            Element::Sync => display.draw_pointer_right(Point::new(4, 32)),
            _ => {}
        };
    }
}
