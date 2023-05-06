use super::{Prob, Pwm, Rate};

#[derive(Clone, Copy)]
pub struct GateState {
    pub rate: Rate,
    pub pwm: Pwm,
    pub prob: Prob,
}

impl Default for GateState {
    fn default() -> Self {
        Self::new()
    }
}

impl GateState {
    pub fn new() -> Self {
        GateState {
            rate: Rate::Unity,
            pwm: Pwm::P50,
            prob: Prob::P100,
        }
    }
}
