use core::fmt::Write;

use embedded_graphics::prelude::{Point, Size};
use heapless::String;
use tinybmp::Bmp as TinyBmp;

use crate::{
    display::Bmp,
    screens::Display,
    state::{Gate, GateState, Pwm, Rate},
    tile_grid::TileGrid,
};

const CLOCK: &[u8; 1318] = include_bytes!("../assets/icons/Clock.bmp");
const PWM: &[u8; 2166] = include_bytes!("../assets/icons/PWMSpritesheetSmol.bmp");

pub struct GateScreen {
    clock: Bmp,
    gate: Gate,
    name: String<3>,
    pwm: Bmp,
    pwm_tile_grid: TileGrid,
}

impl GateScreen {
    pub fn new(gate: Gate) -> Self {
        let mut name = String::new();
        write!(name, "{}", gate).unwrap();

        let clock = TinyBmp::from_slice(CLOCK).unwrap();
        let pwm = TinyBmp::from_slice(PWM).unwrap();
        let pwm_tile_grid = TileGrid::new(Size::new(5, 2), Size::new(13, 8));

        Self {
            clock,
            gate,
            name,
            pwm,
            pwm_tile_grid,
        }
    }

    pub fn draw(&mut self, state: &GateState, display: &mut Display) {
        self.draw_name(display);
        self.draw_clock(display);
        self.draw_rate(state.rate, display);
        self.draw_pwm(state.pwm, display); // 65x16 (13x8)
    }

    fn draw_name(&mut self, display: &mut Display) {
        display.draw_bigge_text(&self.name, Point::new(0, 24));
    }

    fn draw_clock(&mut self, display: &mut Display) {
        display.draw_bmp(&self.clock, Point::new(54, 8));
    }

    fn draw_rate(&mut self, rate: Rate, display: &mut Display) {
        display.draw_smol_text(&rate.into(), Point::new(72, 29));
    }

    fn draw_pwm(&mut self, pwm: Pwm, display: &mut Display) {
        let rectangle = self.pwm_tile_grid.get_rect(pwm.index());
        display.draw_sub_bmp(&self.pwm, &rectangle, Point::new(70, 30));
    }
}
