use core::fmt::Write;

use embedded_graphics::prelude::{Point, Size};
use heapless::String;
use seq::{LaneConfig as OutputConfig, Prob, Pwm, Rate};
use tinybmp::Bmp as TinyBmp;

use crate::{
    display::{Bmp, TileGrid},
    screens::Display,
    state::{Output, ProbString, RateString},
};

const CLOCK: &[u8; 1318] = include_bytes!("../assets/icons/Clock.bmp");
const DICE: &[u8; 1334] = include_bytes!("../assets/icons/Dice.bmp");
const PWM: &[u8; 2166] = include_bytes!("../assets/icons/PWMSpritesheetSmol.bmp");

pub struct GateScreen {
    clock: Bmp,
    dice: Bmp,
    name_str: String<3>,
    pwm: Bmp,
    pwm_tile_grid: TileGrid,
}

impl GateScreen {
    pub fn new() -> Self {
        let clock = TinyBmp::from_slice(CLOCK).unwrap();
        let dice = TinyBmp::from_slice(DICE).unwrap();
        let pwm = TinyBmp::from_slice(PWM).unwrap();
        let pwm_tile_grid = TileGrid::new(Size::new(5, 2), Size::new(13, 8));

        Self {
            clock,
            dice,
            name_str: String::new(),
            pwm,
            pwm_tile_grid,
        }
    }

    pub fn draw(&mut self, name: &Output, config: &OutputConfig, display: &mut Display) {
        self.draw_name(name, display);
        self.draw_clock(display);
        self.draw_dice(display);
        self.draw_rate(config.rate, display);
        self.draw_prob(config.prob, display);
        self.draw_pwm(config.pwm, display); // 65x16 (13x8)
    }

    fn draw_name(&mut self, output: &Output, display: &mut Display) {
        self.name_str.clear();
        write!(self.name_str, "{}", output).unwrap();

        display.draw_bigge_text(&self.name_str, Point::new(0, 24));
    }

    fn draw_clock(&mut self, display: &mut Display) {
        display.draw_bmp(&self.clock, Point::new(54, 8));
    }

    fn draw_dice(&mut self, display: &mut Display) {
        display.draw_bmp(&self.dice, Point::new(54, 46));
    }

    fn draw_rate(&mut self, rate: Rate, display: &mut Display) {
        display.draw_smol_text(&RateString::from(rate).0, Point::new(72, 29));
    }

    fn draw_prob(&mut self, prob: Prob, display: &mut Display) {
        display.draw_smol_text(&ProbString::from(prob).0, Point::new(74, 66));
    }

    fn draw_pwm(&mut self, pwm: Pwm, display: &mut Display) {
        let rectangle = self.pwm_tile_grid.get_rect(pwm.index());
        display.draw_sub_bmp(&self.pwm, &rectangle, Point::new(70, 30));
    }
}
