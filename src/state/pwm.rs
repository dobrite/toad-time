use seq::Pwm;

use super::Updatable;

impl Updatable for Pwm {
    fn next(&mut self) -> Option<Self> {
        let pwm = match self {
            Pwm::P90 => Option::None,
            Pwm::P80 => Option::Some(Pwm::P90),
            Pwm::P70 => Option::Some(Pwm::P80),
            Pwm::P60 => Option::Some(Pwm::P70),
            Pwm::P50 => Option::Some(Pwm::P60),
            Pwm::P40 => Option::Some(Pwm::P50),
            Pwm::P30 => Option::Some(Pwm::P40),
            Pwm::P20 => Option::Some(Pwm::P30),
            Pwm::P10 => Option::Some(Pwm::P20),
            Pwm::Pew => Option::Some(Pwm::P10),
        };

        if let Option::Some(pwm) = pwm {
            *self = pwm;
        };

        pwm
    }

    fn prev(&mut self) -> Option<Self> {
        let pwm = match self {
            Pwm::Pew => Option::None,
            Pwm::P10 => Option::Some(Pwm::Pew),
            Pwm::P20 => Option::Some(Pwm::P10),
            Pwm::P30 => Option::Some(Pwm::P20),
            Pwm::P40 => Option::Some(Pwm::P30),
            Pwm::P50 => Option::Some(Pwm::P40),
            Pwm::P60 => Option::Some(Pwm::P50),
            Pwm::P70 => Option::Some(Pwm::P60),
            Pwm::P80 => Option::Some(Pwm::P70),
            Pwm::P90 => Option::Some(Pwm::P80),
        };

        if let Option::Some(pwm) = pwm {
            *self = pwm;
        };

        pwm
    }
}
