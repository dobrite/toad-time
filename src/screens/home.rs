use core::fmt::Write;

use embedded_graphics::{
    prelude::{Point, Size},
    primitives::Rectangle,
};
use heapless::String;
use tinybmp::Bmp as TinyBmp;

use crate::{display::Bmp, screens::Display, state::Sync};

const FROGGE: &[u8; 4950] = include_bytes!("../assets/icons/SpinSpritesheet.bmp"); // 88x44
const POINTER: &[u8; 630] = include_bytes!("../assets/icons/Pointer.bmp");
const PLAY_PAUSE: &[u8; 1590] = include_bytes!("../assets/icons/PlayPause.bmp");

pub struct Home {
    bpm_label: String<3>,
    bpm_str: String<3>,
    frogge: Bmp,
    play_pause: Bmp,
    pointer: Bmp,
    sync_str: String<3>,
}

impl Home {
    pub fn new() -> Self {
        let bpm_label = String::new();
        let bpm_str = String::new();
        let play_pause = TinyBmp::from_slice(PLAY_PAUSE).unwrap();
        let pointer = TinyBmp::from_slice(POINTER).unwrap();
        let sync_str = String::new();
        let frogge = TinyBmp::from_slice(FROGGE).unwrap();

        Self {
            bpm_label,
            bpm_str,
            frogge,
            play_pause,
            pointer,
            sync_str,
        }
    }

    pub fn draw(&mut self, display: &mut Display) {
        self.draw_bpm(display, 120);
        self.draw_frogge(display);
        self.draw_pointer(display);
        self.draw_sync(display, &Sync::Ext);
        self.draw_play_pause(display, true);
    }

    pub fn draw_bpm(&mut self, display: &mut Display, bpm: u32) {
        self.bpm_label.clear();
        write!(self.bpm_label, "BPM").unwrap();

        display.draw_smol_text(&self.bpm_label, Point::new(68, 27));

        self.bpm_str.clear();
        write!(self.bpm_str, "{}", bpm).unwrap();

        display.draw_bigge_text(&self.bpm_str, Point::new(22, 30));
    }

    fn draw_frogge(&mut self, display: &mut Display) {
        let rectangle = Rectangle::new(Point::new(0, 0), Size::new(22, 22));
        display.draw_sub_bmp(&self.frogge, &rectangle, Point::new(80, 26));
    }

    fn draw_pointer(&mut self, display: &mut Display) {
        display.draw_bmp(&self.pointer, Point::new(4, 8));
    }

    fn draw_sync(&mut self, display: &mut Display, sync: &Sync) {
        self.sync_str.clear();
        write!(self.sync_str, "{}", sync).unwrap();

        display.draw_smol_text(&self.sync_str, Point::new(22, 50));
    }

    fn draw_play_pause(&mut self, display: &mut Display, is_playing: bool) {
        let rectangle = if is_playing {
            Rectangle::new(Point::new(0, 0), Size::new(16, 16))
        } else {
            Rectangle::new(Point::new(16, 0), Size::new(32, 16))
        };

        display.draw_sub_bmp(&self.play_pause, &rectangle, Point::new(56, 30));
    }
}
