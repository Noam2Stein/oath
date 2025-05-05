use crate::*;

pub trait Parse {
    fn parse(parser: &mut Parser, output: &mut Self) -> ParseExit;

    fn parse_error() -> Self;
}

impl Parse for () {
    fn parse(_parser: &mut Parser, _output: &mut Self) -> ParseExit {
        ParseExit::Complete
    }

    fn parse_error() -> Self {
        ()
    }
}

impl<T: Parse> Parse for Box<T> {
    fn parse(parser: &mut Parser, output: &mut Self) -> ParseExit {
        T::parse(parser, &mut **output)
    }

    fn parse_error() -> Self {
        Box::new(T::parse_error())
    }
}
