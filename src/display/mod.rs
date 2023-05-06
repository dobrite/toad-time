use eg_pcf::{include_pcf, text::PcfTextStyle, PcfFont};
use embedded_graphics::{
    image::{Image, ImageDrawableExt},
    pixelcolor::BinaryColor,
    prelude::Point,
    primitives::Rectangle,
    text::Text,
    Drawable,
};
use rp_pico::hal::{
    gpio::{
        pin::{bank0::*, PushPull},
        Output, Pin,
    },
    pac,
    spi::{Enabled, Spi},
};
use ssd1306::Ssd1306;
use tinybmp::Bmp as TinyBmp;

pub use self::tile_grid::TileGrid;

mod tile_grid;

const SMOL_FONT: PcfFont = include_pcf!("src/assets/fonts/FrogPrincess-7.pcf", 'A'..='Z' | 'a'..='z' | '0'..='9' | ' ' | '/' | '%');
const BIGGE_FONT: PcfFont = include_pcf!("src/assets/fonts/FrogPrincess-10.pcf", 'A'..='Z' | 'a'..='z' | '0'..='9' | ' ' | '/' | '%');

pub type Bmp = TinyBmp<'static, BinaryColor>;
pub type Ssd1306Display = Ssd1306<
    ssd1306::prelude::SPIInterface<
        Spi<Enabled, pac::SPI0, 8>,
        Pin<Gpio16, Output<PushPull>>,
        Pin<Gpio17, Output<PushPull>>,
    >,
    ssd1306::prelude::DisplaySize128x64,
    ssd1306::mode::BufferedGraphicsMode<ssd1306::prelude::DisplaySize128x64>,
>;

type Font = PcfTextStyle<'static, BinaryColor>;

pub struct Display {
    display: Ssd1306Display,
    bigge_font: Font,
    smol_font: Font,
}

impl Display {
    pub fn new(display: Ssd1306Display) -> Self {
        let bigge_font = PcfTextStyle::new(&BIGGE_FONT, BinaryColor::On);
        let smol_font = PcfTextStyle::new(&SMOL_FONT, BinaryColor::On);

        Self {
            bigge_font,
            smol_font,
            display,
        }
    }

    pub fn clear(&mut self) {
        self.display.clear();
    }

    pub fn flush(&mut self) {
        self.display.flush().ok();
    }

    pub fn draw_smol_text<S: AsRef<str>>(&mut self, str: S, point: Point) {
        Text::new(str.as_ref(), point, self.smol_font)
            .draw(&mut self.display)
            .unwrap();
    }

    pub fn draw_bigge_text<S: AsRef<str>>(&mut self, str: S, point: Point) {
        Text::new(str.as_ref(), point, self.bigge_font)
            .draw(&mut self.display)
            .unwrap();
    }

    pub fn draw_bmp(&mut self, bmp: &Bmp, point: Point) {
        Image::new(bmp, point).draw(&mut self.display).ok();
    }

    pub fn draw_sub_bmp(&mut self, bmp: &Bmp, rectangle: &Rectangle, point: Point) {
        let sub_bmp = bmp.sub_image(rectangle);
        Image::new(&sub_bmp, point).draw(&mut self.display).ok();
    }
}
