use heapless::String;
use seq::Prob;

use super::Updatable;

impl Updatable for Prob {
    fn next(&self) -> Option<Self> {
        match self {
            Prob::P100 => Option::None,
            Prob::P90 => Option::Some(Prob::P100),
            Prob::P80 => Option::Some(Prob::P90),
            Prob::P70 => Option::Some(Prob::P80),
            Prob::P60 => Option::Some(Prob::P70),
            Prob::P50 => Option::Some(Prob::P60),
            Prob::P40 => Option::Some(Prob::P50),
            Prob::P30 => Option::Some(Prob::P40),
            Prob::P20 => Option::Some(Prob::P30),
            Prob::P10 => Option::Some(Prob::P20),
        }
    }

    fn prev(&self) -> Option<Self> {
        match self {
            Prob::P10 => Option::None,
            Prob::P20 => Option::Some(Prob::P10),
            Prob::P30 => Option::Some(Prob::P20),
            Prob::P40 => Option::Some(Prob::P30),
            Prob::P50 => Option::Some(Prob::P40),
            Prob::P60 => Option::Some(Prob::P50),
            Prob::P70 => Option::Some(Prob::P60),
            Prob::P80 => Option::Some(Prob::P70),
            Prob::P90 => Option::Some(Prob::P80),
            Prob::P100 => Option::Some(Prob::P90),
        }
    }
}

pub struct ProbString(pub String<4>);

impl From<Prob> for ProbString {
    fn from(val: Prob) -> Self {
        let prob_string = match val {
            Prob::P100 => "100%",
            Prob::P90 => "90%",
            Prob::P80 => "80%",
            Prob::P70 => "70%",
            Prob::P60 => "60%",
            Prob::P50 => "50%",
            Prob::P40 => "40%",
            Prob::P30 => "30%",
            Prob::P20 => "20%",
            Prob::P10 => "10%",
        };

        ProbString(prob_string.into())
    }
}
