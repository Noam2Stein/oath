use crate::*;

pub trait OptionParse: Sized {
    fn option_parse(parser: &mut Parser<impl ParserIterator>) -> Option<Self>;

    fn detect(parser: &Parser<impl ParserIterator>) -> bool;

    fn desc() -> &'static str;

    fn try_parse(parser: &mut Parser<impl ParserIterator>) -> Try<Self> {
        if let Some(output) = Self::option_parse(parser) {
            Try::Success(output)
        } else {
            parser.context().push_error(Error::new(
                format!("SyntaxError: expected {}", Self::desc()),
                parser.peek_span(),
            ));

            Try::Failure
        }
    }
}

impl<T: OptionParse> Parse for Option<T> {
    fn parse(parser: &mut Parser<impl ParserIterator>) -> Self {
        T::option_parse(parser)
    }

    fn parse_error() -> Self {
        None
    }
}
