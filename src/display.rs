use core::fmt::Write;
use eg_pcf::{include_pcf, text::PcfTextStyle, PcfFont};
use heapless::String;
use tinybmp::Bmp;

use rp_pico::hal::{
    gpio::pin::bank0::*,
    gpio::pin::PushPull,
    gpio::Output,
    gpio::Pin,
    pac,
    spi::{Enabled, Spi},
};

use embedded_graphics::{
    geometry::{Point, Size},
    image::{Image, ImageDrawableExt},
    pixelcolor::BinaryColor,
    primitives::Rectangle,
    text::Text,
    Drawable,
};

use ssd1306::Ssd1306;

pub type Ssd1306Display = Ssd1306<
    ssd1306::prelude::SPIInterface<
        Spi<Enabled, pac::SPI0, 8>,
        Pin<Gpio16, Output<PushPull>>,
        Pin<Gpio17, Output<PushPull>>,
    >,
    ssd1306::prelude::DisplaySize128x64,
    ssd1306::mode::BufferedGraphicsMode<ssd1306::prelude::DisplaySize128x64>,
>;

use crate::state::{State, StateChange, Sync};

const SMOL_FONT: PcfFont =
    include_pcf!("src/assets/fonts/FrogPrincess-7.pcf", 'A'..='Z' | 'a'..='z' | '0'..='9' | ' ');
const BIGGE_FONT: PcfFont =
    include_pcf!("src/assets/fonts/FrogPrincess-10.pcf", 'A'..='Z' | 'a'..='z' | '0'..='9' | ' ');
const FROGGE: &[u8; 4950] = include_bytes!("assets/icons/SpinSpritesheet.bmp"); // 88x44
const POINTER: &[u8; 630] = include_bytes!("assets/icons/Pointer.bmp");
const PLAY_PAUSE: &[u8; 1590] = include_bytes!("assets/icons/PlayPause.bmp");

pub struct Display {
    display: Ssd1306Display,
    bigge_font: PcfTextStyle<'static, BinaryColor>,
    smol_font: PcfTextStyle<'static, BinaryColor>,
    frogge: Bmp<'static, BinaryColor>,
    play_pause: Bmp<'static, BinaryColor>,
    pointer: Bmp<'static, BinaryColor>,
    bpm_str: heapless::String<3>,
    bpm_label: heapless::String<3>,
    sync_str: heapless::String<3>,
}

impl Display {
    pub fn new(initial_state: State, display: Ssd1306Display) -> Self {
        let bigge_font = PcfTextStyle::new(&BIGGE_FONT, BinaryColor::On);
        let smol_font = PcfTextStyle::new(&SMOL_FONT, BinaryColor::On);
        let frogge: Bmp<BinaryColor> = Bmp::from_slice(FROGGE).unwrap();
        let play_pause: Bmp<BinaryColor> = Bmp::from_slice(PLAY_PAUSE).unwrap();
        let pointer: Bmp<BinaryColor> = Bmp::from_slice(POINTER).unwrap();
        let bpm_str: String<3> = String::new();
        let sync_str: String<3> = String::new();
        let bpm_label: String<3> = String::new();
        let mut display = Self {
            bigge_font,
            smol_font,
            frogge,
            play_pause,
            pointer,
            bpm_str,
            bpm_label,
            sync_str,
            display,
        };
        display.display.clear();
        display.draw_pointer();
        display.draw_bpm(initial_state.bpm());
        display.draw_frogge();
        display.draw_play_pause(initial_state.is_playing());
        display.draw_sync(initial_state.sync());
        display.display.flush().unwrap();
        display
    }

    pub fn handle_state_change(&mut self, state_change: StateChange) {
        self.display.clear();
        match state_change {
            StateChange::Bpm(bpm) => self.draw_bpm(bpm),
            StateChange::None => unreachable!(),
        }
        self.display.flush().unwrap();
    }

    fn draw_bpm(&mut self, bpm: u32) {
        self.bpm_label.clear();
        write!(self.bpm_label, "BPM").unwrap();

        Text::new(&self.bpm_label, Point::new(68, 27), self.smol_font)
            .draw(&mut self.display)
            .unwrap();

        self.bpm_str.clear();
        write!(self.bpm_str, "{}", bpm).unwrap();

        Text::new(&self.bpm_str, Point::new(22, 30), self.bigge_font)
            .draw(&mut self.display)
            .unwrap();
    }

    fn draw_frogge(&mut self) {
        let rectangle = Rectangle::new(Point::new(0, 0), Size::new(22, 22));
        Image::new(&self.frogge.sub_image(&rectangle), Point::new(80, 26))
            .draw(&mut self.display)
            .ok();
    }

    fn draw_pointer(&mut self) {
        Image::new(&self.pointer, Point::new(4, 8))
            .draw(&mut self.display)
            .ok();
    }

    fn draw_sync(&mut self, sync: &Sync) {
        self.sync_str.clear();
        write!(self.sync_str, "{}", sync).unwrap();

        Text::new(&self.sync_str, Point::new(22, 50), self.smol_font)
            .draw(&mut self.display)
            .unwrap();
    }

    fn draw_play_pause(&mut self, is_playing: bool) {
        let rectangle = if is_playing {
            Rectangle::new(Point::new(0, 0), Size::new(16, 16))
        } else {
            Rectangle::new(Point::new(16, 0), Size::new(32, 16))
        };

        Image::new(&self.play_pause.sub_image(&rectangle), Point::new(56, 30))
            .draw(&mut self.display)
            .ok();
    }
}
