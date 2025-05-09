use std::ops::{BitOr, BitOrAssign};

use super::*;

pub trait OptionParse: Sized {
    fn option_parse(parser: &mut Parser<impl Tokenizer>, output: &mut Option<Self>) -> ParseExit;

    fn detect(parser: &Parser<impl InnerTokenizer>) -> Detection;
}

pub trait Parse: OptionParse {
    fn parse(parser: &mut Parser<impl InnerTokenizer>, output: &mut Self) -> ParseExit;

    fn parse_error() -> Self;
}

pub trait ParseDesc: OptionParse {
    fn desc() -> &'static str;
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

impl<T: OptionParse> OptionParse for Option<T> {
    fn option_parse(parser: &mut Parser<impl InnerTokenizer>, output: &mut Option<Self>) -> ParseExit {
        let mut option = None;
        let exit = T::option_parse(parser, &mut option);

        *output = Some(option);

        exit
    }

    fn detect(parser: &Parser<impl InnerTokenizer>) -> Detection {
        match T::detect(parser) {
            Detection::Detected => Detection::Detected,
            Detection::NotDetected => Detection::EmptyDetected,
            Detection::EmptyDetected => Detection::EmptyDetected,
        }
    }
}
impl<T: OptionParse> Parse for Option<T> {
    fn parse(parser: &mut Parser<impl InnerTokenizer>, output: &mut Self) -> ParseExit {
        T::option_parse(parser, output)
    }

    fn parse_error() -> Self {
        None
    }
}

impl OptionParse for () {
    fn option_parse(_parser: &mut Parser<impl InnerTokenizer>, _output: &mut Option<Self>) -> ParseExit {
        ParseExit::Complete
    }

    fn detect(_parser: &Parser<impl InnerTokenizer>) -> Detection {
        Detection::EmptyDetected
    }
}
impl Parse for () {
    fn parse(_parser: &mut Parser<impl InnerTokenizer>, _output: &mut Self) -> ParseExit {
        ParseExit::Complete
    }

    fn parse_error() -> Self {
        ()
    }
}

impl<T: OptionParse> OptionParse for Box<T> {
    fn option_parse(parser: &mut Parser<impl InnerTokenizer>, output: &mut Option<Self>) -> ParseExit {
        let mut inner = None;

        let exit = T::option_parse(parser, &mut inner);

        if let Some(inner) = inner {
            *output = Some(Box::new(inner));
        }

        exit
    }

    fn detect(parser: &Parser<impl InnerTokenizer>) -> Detection {
        T::detect(parser)
    }
}
impl<T: Parse> Parse for Box<T> {
    fn parse(parser: &mut Parser<impl InnerTokenizer>, output: &mut Self) -> ParseExit {
        let mut inner = T::parse_error();

        let exit = T::parse(parser, &mut inner);

        *output = Box::new(inner);

        exit
    }

    fn parse_error() -> Self {
        Box::new(T::parse_error())
    }
}
impl<T: ParseDesc> ParseDesc for Box<T> {
    fn desc() -> &'static str {
        T::desc()
    }
}
