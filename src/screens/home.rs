use core::fmt::Write;

use embedded_graphics::{
    prelude::{Point, Size},
    primitives::Rectangle,
};
use heapless::String;
use tinybmp::Bmp as TinyBmp;

use crate::{
    display::Bmp,
    screens::Display,
    state::{Bpm, PlayStatus, State, Sync},
    StateChange,
};

const FROGGE: &[u8; 4950] = include_bytes!("../assets/icons/spin-sprite-sheet.bmp"); // 88x44
const PLAY_PAUSE: &[u8; 1590] = include_bytes!("../assets/icons/play-pause.bmp");

pub struct HomeScreen {
    bpm_label: String<3>,
    bpm_str: String<3>,
    frogge: Bmp,
    play_pause: Bmp,
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
        let play_pause = TinyBmp::from_slice(PLAY_PAUSE).unwrap();
        let sync_str = String::new();
        let frogge = TinyBmp::from_slice(FROGGE).unwrap();

        Self {
            bpm_label,
            bpm_str,
            frogge,
            play_pause,
            sync_str,
        }
    }

    pub fn draw(&mut self, state_change: &StateChange, state: &State, display: &mut Display) {
        match state_change {
            StateChange::Bpm(bpm) => {
                self.clear_bpm_value(display);
                self.draw_bpm_value(display, bpm);
            }
            StateChange::Sync(sync) => {
                self.clear_sync(display);
                self.draw_sync(display, sync);
            }
            StateChange::PlayStatus(play_status) => self.draw_play_pause(display, play_status),
            StateChange::NextScreen(_) => {
                display.clear();
                self.draw_bpm_label(display);
                self.draw_bpm_value(display, &state.bpm);
                self.draw_frogge(display);
                self.draw_sync(display, &state.sync);
                self.draw_play_pause(display, &state.play_status);
            }
            _ => {}
        }
    }

    fn draw_bpm_label(&mut self, display: &mut Display) {
        self.bpm_label.clear();
        write!(self.bpm_label, "BPM").unwrap();
        display.draw_smol_text(&self.bpm_label, Point::new(68, 27));
    }

    fn clear_bpm_value(&mut self, display: &mut Display) {
        display.clear_bigge_text(&self.bpm_str, Point::new(22, 30));
    }

    fn draw_bpm_value(&mut self, display: &mut Display, bpm: &Bpm) {
        self.bpm_str.clear();
        write!(self.bpm_str, "{}", bpm).unwrap();
        display.draw_bigge_text(&self.bpm_str, Point::new(22, 30));
    }

    fn draw_frogge(&mut self, display: &mut Display) {
        let size = Size::new(22, 22);
        let rectangle = Rectangle::new(Point::zero(), size);
        let point = Point::new(80, 26);
        display.clear_rect(Rectangle::new(point, size));
        display.draw_sub_bmp(&self.frogge, &rectangle, point);
    }

    fn clear_sync(&mut self, display: &mut Display) {
        display.clear_smol_text(&self.sync_str, Point::new(22, 50));
    }

    fn draw_sync(&mut self, display: &mut Display, sync: &Sync) {
        self.sync_str.clear();
        write!(self.sync_str, "{}", sync).unwrap();

        display.draw_smol_text(&self.sync_str, Point::new(22, 50));
    }

    fn draw_play_pause(&mut self, display: &mut Display, play_status: &PlayStatus) {
        let size = Size::new(16, 16);
        let rectangle = match play_status {
            PlayStatus::Playing => Rectangle::new(Point::zero(), size),
            PlayStatus::Paused => Rectangle::new(Point::new(16, 0), Size::new(32, 16)),
        };

        let point = Point::new(56, 30);
        display.clear_rect(Rectangle::new(point, size));
        display.draw_sub_bmp(&self.play_pause, &rectangle, point);
    }
}
