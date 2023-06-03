use core::fmt::Write;

use embedded_graphics::prelude::{Point, Size};
use heapless::String;
use seq::{OutputConfig, OutputType, Prob, Pwm, Rate};
use tinybmp::Bmp as TinyBmp;

use crate::{
    display::{Bmp, TileGrid},
    screens::Display,
    state::{Output, OutputTypeString, ProbString, RateString, Screen},
    StateChange,
};

const CLOCK: &[u8; 1318] = include_bytes!("../assets/icons/clock.bmp");
const DICE: &[u8; 1334] = include_bytes!("../assets/icons/die.bmp");
const PWM: &[u8; 12_598] = include_bytes!("../assets/icons/pwm-sprite-sheet.bmp");

pub struct GateScreen {
    clock: Bmp,
    dice: Bmp,
    name_str: String<3>,
    output_type_str: String<3>,
    prob_str: String<4>,
    pwm: Bmp,
    pwm_tile_grid: TileGrid,
    rate_str: String<3>,
}

impl GateScreen {
    pub fn new() -> Self {
        let clock = TinyBmp::from_slice(CLOCK).unwrap();
        let dice = TinyBmp::from_slice(DICE).unwrap();
        let pwm = TinyBmp::from_slice(PWM).unwrap();
        let pwm_tile_grid = TileGrid::new(Size::new(5, 2), Size::new(26, 16));

        Self {
            clock,
            dice,
            name_str: String::new(),
            output_type_str: String::new(),
            prob_str: String::new(),
            pwm,
            pwm_tile_grid,
            rate_str: String::new(),
        }
    }

    pub fn draw(
        &mut self,
        state_change: &StateChange,
        config: &OutputConfig,
        display: &mut Display,
    ) {
        match state_change {
            StateChange::Rate(_, rate) => {
                self.clear_rate(display);
                self.draw_rate(rate, display);
            }
            StateChange::Prob(_, prob) => {
                self.clear_prob(display);
                self.draw_prob(prob, display);
            }
            StateChange::Pwm(_, pwm) => {
                self.draw_pwm(pwm, display);
            }
            StateChange::OutputType(output, _) => {
                display.clear();
                self.draw_name(output, display);
                self.draw_clock(display);
                self.draw_dice(display);
                self.draw_rate(&config.rate(), display);
                self.draw_prob(&config.prob(), display);
                self.draw_pwm(&config.pwm(), display); // 65x16 (13x8)
                self.draw_output_type(&config.output_type(), display);
            }
            StateChange::NextScreen(Screen::Output(output, _)) => {
                display.clear();
                self.draw_name(output, display);
                self.draw_clock(display);
                self.draw_dice(display);
                self.draw_rate(&config.rate(), display);
                self.draw_prob(&config.prob(), display);
                self.draw_pwm(&config.pwm(), display); // 65x16 (13x8)
                self.draw_output_type(&config.output_type(), display);
            }
            _ => {}
        }
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
        display.draw_bmp(&self.dice, Point::new(54, 26));
    }

    fn clear_rate(&mut self, display: &mut Display) {
        display.clear_smol_text(&self.rate_str, Point::new(72, 29));
    }

    fn draw_rate(&mut self, rate: &Rate, display: &mut Display) {
        self.rate_str.clear();
        write!(self.rate_str, "{}", RateString::from(rate).0).unwrap();
        display.draw_smol_text(&self.rate_str, Point::new(72, 29));
    }

    fn clear_prob(&mut self, display: &mut Display) {
        display.clear_smol_text(&self.prob_str, Point::new(74, 46));
    }

    fn draw_prob(&mut self, prob: &Prob, display: &mut Display) {
        self.prob_str.clear();
        write!(self.prob_str, "{}", ProbString::from(prob).0).unwrap();
        display.draw_smol_text(&self.prob_str, Point::new(74, 46));
    }

    fn draw_pwm(&mut self, pwm: &Pwm, display: &mut Display) {
        let rectangle = self.pwm_tile_grid.get_rect(pwm.index());
        let point = Point::new(55, 46);
        display.clear_sub_bmp(&self.pwm, &rectangle, point);
        display.draw_sub_bmp(&self.pwm, &rectangle, point);
    }

    fn draw_output_type(&mut self, output_type: &OutputType, display: &mut Display) {
        self.output_type_str.clear();
        let str = OutputTypeString::from(output_type).0;
        write!(self.output_type_str, "{}", str).unwrap();
        display.draw_bigge_text(&self.output_type_str, Point::new(0, 50));
    }
}
