use fugit::RateExtU32;

use crate::MicroSeconds;

const MAX_MULT: u32 = 192;
const PWM_PERCENT_INCREMENTS: u32 = 10;
const SECONDS_IN_MINUTES: f32 = 60.0;

pub fn resolution() -> u32 {
    PWM_PERCENT_INCREMENTS * MAX_MULT
}

pub fn tick_duration(bpm: f32) -> MicroSeconds {
    let bps = bpm / SECONDS_IN_MINUTES;
    const MULTIPLYER: f32 = (PWM_PERCENT_INCREMENTS * MAX_MULT) as f32;
    let hertz: u32 = (bps * MULTIPLYER) as u32;

    hertz.Hz::<1, 1>().into_duration().into()
}
