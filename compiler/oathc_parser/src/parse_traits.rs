use std::ops::{BitOr, BitOrAssign};

use super::*;

pub trait OptionParse: Sized {
    fn option_parse(parser: &mut impl Tokenizer, output: &mut Option<Self>) -> ParseExit;

    fn detect(parser: &impl Tokenizer) -> Detection;
}

pub trait Parse: OptionParse {
    fn parse(parser: &mut impl Tokenizer, output: &mut Self) -> ParseExit;

    fn parse_error() -> Self;
}

pub trait ParseDesc: OptionParse {
    fn desc() -> &'static str;
}

pub trait ParseFrame: Sized {
    fn option_parse<Inner, T: Tokenizer>(
        parser: &mut T,
        output: &mut Option<(Self, Inner)>,
        parse_outside: impl FnOnce(&mut T) -> (Inner, ParseExit),
        parse_inside: impl FnOnce(&mut GroupTokenizer) -> (Inner, ParseExit),
    ) -> ParseExit;

    fn detect(parser: &impl Tokenizer) -> Detection;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParseExit {
    Complete,
    Cut,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Detection {
    Detected,
    NotDetected,
    EmptyDetected,
}

impl BitOr for Detection {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Detected, _) => Self::Detected,
            (_, Self::Detected) => Self::Detected,
            (Self::EmptyDetected, _) => Self::Detected,
            (_, Self::EmptyDetected) => Self::Detected,
            (Self::NotDetected, Self::NotDetected) => Self::NotDetected,
        }
    }
}
impl BitOrAssign for Detection {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.bitor(rhs)
    }
}
