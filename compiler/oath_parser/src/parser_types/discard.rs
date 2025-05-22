use std::{fmt::Debug, marker::PhantomData};

use super::*;

pub struct Discard<T>(PhantomData<T>);

impl<T: OptionParse> OptionParse for Discard<T> {
    fn option_parse(parser: &mut Parser<impl Tokenizer>, output: &mut Option<Self>) -> ParseExit {
        let mut inner = None;

        let exit = T::option_parse(parser, &mut inner);

        if inner.is_some() {
            *output = Some(Self(PhantomData));
        }

        exit
    }

    fn detect(parser: &Parser<impl Tokenizer>) -> Detection {
        T::detect(parser)
    }
}
impl<T: Parse> Parse for Discard<T> {
    fn parse(parser: &mut Parser<impl Tokenizer>, _output: &mut Self) -> ParseExit {
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

impl<F: ParseFrame> ParseFrame for Discard<F> {
    fn option_parse<P, T: Tokenizer>(
        parser: &mut Parser<T>,
        parse_t: impl FnOnce(&mut Parser<T>) -> (P, ParseExit),
        parse_group: impl FnOnce(&mut Parser<GroupTokenizer>) -> (P, ParseExit),
        output: &mut Option<(Self, P)>,
    ) -> ParseExit {
        let mut value = None;
        let parse_exit = F::option_parse(parser, parse_t, parse_group, &mut value);

        *output = value.map(|(_, value)| (Self(PhantomData), value));

        parse_exit
    }

    fn detect(parser: &Parser<impl Tokenizer>) -> Detection {
        F::detect(parser)
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
