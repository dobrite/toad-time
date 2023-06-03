use embedded_graphics::prelude::Point;
use heapless::String;
use seq::{OutputConfig, OutputType, Prob, Pwm, Rate};

use crate::{
    screens::Display,
    state::{Output, OutputTypeString, ProbString, RateString, Screen},
    StateChange,
};

pub struct GateScreen {
    name_str: String<3>,
    output_type_str: String<3>,
    prob_str: String<4>,
    rate_str: String<3>,
}

impl GateScreen {
    pub fn new() -> Self {
        Self {
            name_str: String::new(),
            output_type_str: String::new(),
            prob_str: String::new(),
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
                self.draw_rate(display, rate);
            }
            StateChange::Prob(_, prob) => {
                self.clear_prob(display);
                self.draw_prob(display, prob);
            }
            StateChange::Pwm(_, pwm) => {
                self.draw_pwm(display, pwm);
            }
            StateChange::OutputType(output, _) => {
                display.clear();
                self.draw_screen(display, output, config);
            }
            StateChange::NextScreen(Screen::Output(output, _)) => {
                display.clear();
                self.draw_screen(display, output, config);
            }
            _ => {}
        }
    }

    fn draw_screen(&mut self, display: &mut Display, output: &Output, config: &OutputConfig) {
        self.draw_name(display, output);
        self.draw_clock(display);
        self.draw_dice(display);
        self.draw_rate(display, &config.rate());
        self.draw_prob(display, &config.prob());
        self.draw_pwm(display, &config.pwm()); // 65x16 (13x8)
        self.draw_output_type(display, &config.output_type());
    }

    fn draw_name(&mut self, display: &mut Display, output: &Output) {
        display.draw_bigge_text(&mut self.name_str, output, Point::new(0, 24));
    }

    fn draw_clock(&mut self, display: &mut Display) {
        display.draw_clock(Point::new(54, 8));
    }

    fn draw_dice(&mut self, display: &mut Display) {
        display.draw_dice(Point::new(54, 26));
    }

    fn clear_rate(&mut self, display: &mut Display) {
        display.clear_smol_text(&self.rate_str, Point::new(72, 29));
    }

    fn draw_rate(&mut self, display: &mut Display, rate: &Rate) {
        let str = RateString::from(rate).0;
        display.draw_smol_text(&mut self.rate_str, str, Point::new(72, 29));
    }

    fn clear_prob(&mut self, display: &mut Display) {
        display.clear_smol_text(&self.prob_str, Point::new(74, 46));
    }

    fn draw_prob(&mut self, display: &mut Display, prob: &Prob) {
        let str = ProbString::from(prob).0;
        display.draw_smol_text(&mut self.prob_str, str, Point::new(74, 46));
    }

    fn draw_pwm(&mut self, display: &mut Display, pwm: &Pwm) {
        let point = Point::new(55, 46);
        display.clear_pwm(point);
        display.draw_pwm(pwm.index(), point);
    }

    fn draw_output_type(&mut self, display: &mut Display, output_type: &OutputType) {
        let str = OutputTypeString::from(output_type).0;
        display.draw_bigge_text(&mut self.output_type_str, str, Point::new(0, 50));
    }
}
