use heapless::String;
use seq::Rate;

use super::Updatable;

const RATES: [Rate; 17] = [
    Rate::Div(64),
    Rate::Div(32),
    Rate::Div(16),
    Rate::Div(8),
    Rate::Div(5),
    Rate::Div(4),
    Rate::Div(3),
    Rate::Div(2),
    Rate::Unity,
    Rate::Mult(2),
    Rate::Mult(3),
    Rate::Mult(4),
    Rate::Mult(5),
    Rate::Mult(8),
    Rate::Mult(16),
    Rate::Mult(32),
    Rate::Mult(64),
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

impl From<Rate> for RateString {
    fn from(val: Rate) -> Self {
        let rate_string = match val {
            Rate::Div(64) => "/64",
            Rate::Div(32) => "/32",
            Rate::Div(16) => "/16",
            Rate::Div(8) => "/8",
            Rate::Div(5) => "/5",
            Rate::Div(4) => "/4",
            Rate::Div(3) => "/3",
            Rate::Div(2) => "/2",
            Rate::Unity => "x1",
            Rate::Mult(2) => "x2",
            Rate::Mult(3) => "x3",
            Rate::Mult(4) => "x4",
            Rate::Mult(5) => "x5",
            Rate::Mult(8) => "x8",
            Rate::Mult(16) => "x16",
            Rate::Mult(32) => "x32",
            Rate::Mult(64) => "x64",
            _ => unreachable!(),
        };

        RateString(rate_string.into())
    }
}
