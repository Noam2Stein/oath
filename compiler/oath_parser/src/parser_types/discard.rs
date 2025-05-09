use std::{fmt::Debug, marker::PhantomData};

use super::*;

pub struct Discard<T>(PhantomData<T>);

impl<T: OptionParse> OptionParse for Discard<T> {
    fn option_parse(parser: &mut Parser<impl InnerTokenizer>, _output: &mut Option<Self>) -> ParseExit {
        let mut inner = None;

        T::option_parse(parser, &mut inner)
    }

    fn detect(parser: &Parser<impl InnerTokenizer>) -> Detection {
        T::detect(parser)
    }
}
impl<T: Parse> Parse for Discard<T> {
    fn parse(parser: &mut Parser<impl InnerTokenizer>, _output: &mut Self) -> ParseExit {
        let mut inner = T::parse_error();

        T::parse(parser, &mut inner)
    }

    fn parse_error() -> Self {
        Self(PhantomData)
    }
}
impl<T: ParseDesc> ParseDesc for Discard<T> {
    fn desc() -> &'static str {
        T::desc()
    }
}

impl<T> Clone for Discard<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Copy for Discard<T> {}

impl<T> Debug for Discard<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Discard")
    }
}
