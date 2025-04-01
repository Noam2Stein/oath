use crate::*;

pub trait Parse {
    fn parse(parser: &mut Parser<impl ParserIterator>) -> Self;

    fn parse_error() -> Self;
}

impl Parse for () {
    fn parse(_parser: &mut Parser<impl ParserIterator>) -> Self {
        ()
    }

    fn parse_error() -> Self {
        ()
    }
}

impl<T: Parse> Parse for Box<T> {
    fn parse(parser: &mut Parser<impl ParserIterator>) -> Self {
        Box::new(T::parse(parser))
    }

    fn parse_error() -> Self {
        Box::new(T::parse_error())
    }
}
