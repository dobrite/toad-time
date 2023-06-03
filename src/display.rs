use embassy_rp::{
    gpio::Output,
    peripherals::{PIN_16, PIN_17, SPI0},
    spi::Spi,
};
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::Dimensions,
    image::{Image, ImageDrawableExt},
    pixelcolor::BinaryColor,
    prelude::{Point, Size},
    primitives::Rectangle,
    Drawable,
};
use embedded_hal_async::spi::ExclusiveDevice;
use ssd1306_async::{
    mode::{BufferedGraphicsMode, DisplayConfig},
    prelude::{DisplaySize128x64, SPIInterface},
    Ssd1306,
};

pub use self::tile_grid::TileGrid;
use self::{
    bmps::{Bmp, Bmps},
    fonts::Fonts,
};

mod bmps;
mod fonts;
mod tile_grid;

pub type Ssd1306Display = Ssd1306<
    SPIInterface<
        ExclusiveDevice<Spi<'static, SPI0, embassy_rp::spi::Async>, Output<'static, PIN_17>>,
        Output<'static, PIN_16>,
    >,
    DisplaySize128x64,
    BufferedGraphicsMode<DisplaySize128x64>,
>;

pub struct Display {
    bmps: Bmps,
    display: Ssd1306Display,
    fonts: Fonts,
    frogge_tile_grid: TileGrid,
    play_pause_tile_grid: TileGrid,
    pwm_tile_grid: TileGrid,
}

impl Display {
    pub fn new(display: Ssd1306Display) -> Self {
        let frogge_tile_grid = TileGrid::new(Size::new(4, 2), Size::new(22, 22));
        let play_pause_tile_grid = TileGrid::new(Size::new(2, 1), Size::new(16, 16));
        let pwm_tile_grid = TileGrid::new(Size::new(5, 2), Size::new(26, 16));

        Self {
            bmps: Bmps::new(),
            display,
            fonts: Fonts::new(),
            frogge_tile_grid,
            play_pause_tile_grid,
            pwm_tile_grid,
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

    pub fn clear_caret(&mut self, point: Point) {
        let bmp = self.bmps.caret;
        self.clear_bmp(&bmp, point);
    }

    pub fn clear_frogge(&mut self, point: Point) {
        let rectangle = self.frogge_tile_grid.get_rect(0);
        let bmp = self.bmps.frogge;
        self.clear_sub_bmp(&bmp, rectangle, point);
    }

    pub fn clear_play_pause(&mut self, point: Point) {
        let rectangle = self.play_pause_tile_grid.get_rect(0);
        let bmp = self.bmps.play_pause;
        self.clear_sub_bmp(&bmp, rectangle, point);
    }

    pub fn clear_pwm(&mut self, point: Point) {
        let rectangle = self.pwm_tile_grid.get_rect(0);
        let bmp = self.bmps.pwm;
        self.clear_sub_bmp(&bmp, rectangle, point);
    }

    pub fn clear_step_on(&mut self, point: Point) {
        let bmp = self.bmps.step_on;
        self.clear_bmp(&bmp, point);
    }

    pub fn draw_caret(&mut self, point: Point) {
        let bmp = self.bmps.caret;
        self.draw_bmp(&bmp, point);
    }

    pub fn draw_clock(&mut self, point: Point) {
        let bmp = self.bmps.clock;
        self.draw_bmp(&bmp, point);
    }

    pub fn draw_dice(&mut self, point: Point) {
        let bmp = self.bmps.dice;
        self.draw_bmp(&bmp, point);
    }

    pub fn draw_frogge(&mut self, index: usize, point: Point) {
        let rectangle = self.frogge_tile_grid.get_rect(index);
        let bmp = self.bmps.frogge;
        self.draw_sub_bmp(&bmp, rectangle, point);
    }

    pub fn draw_play_pause(&mut self, index: usize, point: Point) {
        let rectangle = self.play_pause_tile_grid.get_rect(index);
        let bmp = self.bmps.play_pause;
        self.draw_sub_bmp(&bmp, rectangle, point);
    }

    pub fn draw_pointer_left(&mut self, point: Point) {
        let bmp = self.bmps.pointer_left;
        self.draw_bmp(&bmp, point);
    }

    pub fn draw_pointer_right(&mut self, point: Point) {
        let bmp = self.bmps.pointer_right;
        self.draw_bmp(&bmp, point);
    }

    pub fn draw_pwm(&mut self, index: usize, point: Point) {
        let rectangle = self.pwm_tile_grid.get_rect(index);
        let bmp = self.bmps.pwm;
        self.draw_sub_bmp(&bmp, rectangle, point);
    }

    pub fn draw_step_off(&mut self, point: Point) {
        let bmp = self.bmps.step_off;
        self.draw_bmp(&bmp, point);
    }

    pub fn draw_step_on(&mut self, point: Point) {
        let bmp = self.bmps.step_on;
        self.draw_bmp(&bmp, point);
    }

    pub fn clear_smol_text<S: AsRef<str>>(&mut self, str: S, point: Point) {
        let bb = self.fonts.smol_font_bounding_box(str.as_ref(), point);
        self.clear_rect(bb);
    }

    pub fn draw_smol_text<S: AsRef<str>>(&mut self, str: S, point: Point) {
        self.fonts
            .smol_text(str.as_ref(), point)
            .draw(&mut self.display)
            .unwrap();
    }

    pub fn clear_bigge_text<S: AsRef<str>>(&mut self, str: S, point: Point) {
        let bb = self.fonts.bigge_font_bounding_box(str.as_ref(), point);
        self.clear_rect(bb);
    }

    pub fn draw_bigge_text<S: AsRef<str>>(&mut self, str: S, point: Point) {
        self.fonts
            .bigge_text(str.as_ref(), point)
            .draw(&mut self.display)
            .unwrap();
    }

    fn clear_bmp(&mut self, bmp: &Bmp, point: Point) {
        let mut bb = bmp.bounding_box();
        bb.top_left = point;
        self.clear_rect(bb);
    }

    fn clear_rect(&mut self, rect: Rectangle) {
        self.display.fill_solid(&rect, BinaryColor::Off).ok();
    }

    fn clear_sub_bmp(&mut self, bmp: &Bmp, rectangle: Rectangle, point: Point) {
        let sub_bmp = bmp.sub_image(&rectangle);
        let mut bb = sub_bmp.bounding_box();
        bb.top_left = point;
        self.clear_rect(bb);
    }

    fn draw_bmp(&mut self, bmp: &Bmp, point: Point) {
        Image::new(bmp, point).draw(&mut self.display).ok();
    }

    fn draw_sub_bmp(&mut self, bmp: &Bmp, rectangle: Rectangle, point: Point) {
        let sub_bmp = bmp.sub_image(&rectangle);
        Image::new(&sub_bmp, point).draw(&mut self.display).ok();
    }
}
