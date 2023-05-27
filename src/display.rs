use eg_pcf::{include_pcf, text::PcfTextStyle, PcfFont};
use embassy_rp::{
    gpio::Output,
    peripherals::{PIN_16, PIN_17, SPI0},
    spi::Spi,
};
use embedded_graphics::{
    image::{Image, ImageDrawableExt},
    pixelcolor::BinaryColor,
    prelude::Point,
    primitives::Rectangle,
    text::Text,
    Drawable,
};
use embedded_hal_async::spi::ExclusiveDevice;
use ssd1306_async::{
    mode::{BufferedGraphicsMode, DisplayConfig},
    prelude::{DisplaySize128x64, SPIInterface},
    Ssd1306,
};
use tinybmp::Bmp as TinyBmp;

pub use self::tile_grid::TileGrid;

mod tile_grid;

const SMOL_FONT: PcfFont = include_pcf!("src/assets/fonts/FrogPrincess-7.pcf", 'A'..='Z' | 'a'..='z' | '0'..='9' | ' ' | '/' | '%');
const BIGGE_FONT: PcfFont = include_pcf!("src/assets/fonts/FrogPrincess-10.pcf", 'A'..='Z' | 'a'..='z' | '0'..='9' | ' ' | '/' | '%');

pub type Bmp = TinyBmp<'static, BinaryColor>;
pub type Ssd1306Display = Ssd1306<
    SPIInterface<
        ExclusiveDevice<Spi<'static, SPI0, embassy_rp::spi::Async>, Output<'static, PIN_17>>,
        Output<'static, PIN_16>,
    >,
    DisplaySize128x64,
    BufferedGraphicsMode<DisplaySize128x64>,
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

    pub async fn init(&mut self) {
        self.display.init().await.ok();
    }

    pub fn clear(&mut self) {
        self.display.clear();
    }

    pub async fn flush(&mut self) {
        self.display.flush().await.ok();
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
