use crate::*;

pub trait ParseDesc: Sized {
    fn desc() -> &'static str;
}
pub trait Parse: ParseDesc {
    fn parse(parser: &mut Parser<impl ParserIterator>) -> Self;
}
pub trait Detect: ParseDesc {
    fn detect(parser: &Parser<impl ParserIterator>) -> bool;
}

impl ParseDesc for () {
    fn desc() -> &'static str {
        "nothing"
    }
}
impl Parse for () {
    fn parse(_parser: &mut Parser<impl ParserIterator>) -> Self {
        ()
    }
}

impl<T: ParseDesc> ParseDesc for Box<T> {
    fn desc() -> &'static str {
        T::desc()
    }
}
impl<T: Parse> Parse for Box<T> {
    fn parse(parser: &mut Parser<impl ParserIterator>) -> Self {
        Box::new(T::parse(parser))
    }
}
impl<T: Detect> Detect for Box<T> {
    fn detect(parser: &Parser<impl ParserIterator>) -> bool {
        T::detect(parser)
    }
}
