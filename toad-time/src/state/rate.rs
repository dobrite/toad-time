use heapless::String;
use seq::Rate;

use super::Updatable;

const RATES: [Rate; 17] = [
    Rate::Div(64.0),
    Rate::Div(32.0),
    Rate::Div(16.0),
    Rate::Div(8.0),
    Rate::Div(5.0),
    Rate::Div(4.0),
    Rate::Div(3.0),
    Rate::Div(2.0),
    Rate::Unity,
    Rate::Mult(2.0),
    Rate::Mult(3.0),
    Rate::Mult(4.0),
    Rate::Mult(5.0),
    Rate::Mult(8.0),
    Rate::Mult(16.0),
    Rate::Mult(32.0),
    Rate::Mult(64.0),
];

impl Updatable for Rate {
    fn next(&self) -> Option<Self> {
        if self == RATES.last().unwrap() {
            Option::None
        } else {
            let index = RATES.iter().position(|r| r == self).unwrap() + 1;
            Option::Some(RATES[index])
        }
    }

    fn prev(&self) -> Option<Self> {
        if self == RATES.first().unwrap() {
            Option::None
        } else {
            let index = RATES.iter().position(|r| r == self).unwrap() - 1;
            Option::Some(RATES[index])
        }
    }
}

pub struct RateString(pub String<3>);

impl From<Rate> for RateString {
    #[allow(illegal_floating_point_literal_pattern)]
    fn from(val: Rate) -> Self {
        let rate_string = match val {
            Rate::Div(64.0) => "/64",
            Rate::Div(32.0) => "/32",
            Rate::Div(16.0) => "/16",
            Rate::Div(8.0) => "/8",
            Rate::Div(5.0) => "/5",
            Rate::Div(4.0) => "/4",
            Rate::Div(3.0) => "/3",
            Rate::Div(2.0) => "/2",
            Rate::Unity => "x1",
            Rate::Mult(2.0) => "x2",
            Rate::Mult(3.0) => "x3",
            Rate::Mult(4.0) => "x4",
            Rate::Mult(5.0) => "x5",
            Rate::Mult(8.0) => "x8",
            Rate::Mult(16.0) => "x16",
            Rate::Mult(32.0) => "x32",
            Rate::Mult(64.0) => "x64",
            _ => unreachable!(),
        };

        RateString(rate_string.into())
    }
}
