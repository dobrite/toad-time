use heapless::String;
use seq::{Frac, Rate};

use super::Updatable;

const RATES: [Rate; 15] = [
    Rate::Div(64, Frac::Zero),
    Rate::Div(32, Frac::Zero),
    Rate::Div(16, Frac::Zero),
    Rate::Div(8, Frac::Zero),
    Rate::Div(5, Frac::Zero),
    Rate::Div(4, Frac::Zero),
    Rate::Div(3, Frac::Zero),
    Rate::Div(2, Frac::Zero),
    Rate::Unity,
    Rate::Mult(2, Frac::Zero),
    Rate::Mult(3, Frac::Zero),
    Rate::Mult(4, Frac::Zero),
    Rate::Mult(5, Frac::Zero),
    Rate::Mult(8, Frac::Zero),
    Rate::Mult(16, Frac::Zero),
];

impl Updatable for Rate {
    fn next(&mut self) -> Option<Self> {
        if self == RATES.last().unwrap() {
            Option::None
        } else {
            let index = RATES.iter().position(|r| r == self).unwrap() + 1;
            *self = RATES[index];
            Option::Some(*self)
        }
    }

    fn prev(&mut self) -> Option<Self> {
        if self == RATES.first().unwrap() {
            Option::None
        } else {
            let index = RATES.iter().position(|r| r == self).unwrap() - 1;
            *self = RATES[index];
            Option::Some(*self)
        }
    }
}

pub struct RateString(pub String<3>);

impl From<&Rate> for RateString {
    fn from(val: &Rate) -> Self {
        let rate_string = match val {
            Rate::Div(64, Frac::Zero) => "/64",
            Rate::Div(32, Frac::Zero) => "/32",
            Rate::Div(16, Frac::Zero) => "/16",
            Rate::Div(8, Frac::Zero) => "/8",
            Rate::Div(5, Frac::Zero) => "/5",
            Rate::Div(4, Frac::Zero) => "/4",
            Rate::Div(3, Frac::Zero) => "/3",
            Rate::Div(2, Frac::Zero) => "/2",
            Rate::Unity => "x1",
            Rate::Mult(2, Frac::Zero) => "x2",
            Rate::Mult(3, Frac::Zero) => "x3",
            Rate::Mult(4, Frac::Zero) => "x4",
            Rate::Mult(5, Frac::Zero) => "x5",
            Rate::Mult(8, Frac::Zero) => "x8",
            Rate::Mult(16, Frac::Zero) => "x16",
            _ => unreachable!(),
        };

        RateString(rate_string.into())
    }
}
