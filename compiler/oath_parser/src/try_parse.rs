use crate::*;

pub trait TryParse: ParseDesc {
    fn try_parse(parser: &mut Parser<impl ParserIterator>) -> Try<Self>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Try<T> {
    Success(T),
    Failure,
}

impl<T: OptionParse> TryParse for T {
    fn try_parse(parser: &mut Parser<impl ParserIterator>) -> Try<Self> {
        if let Some(output) = T::option_parse(parser) {
            Try::Success(output)
        } else {
            parser.context().push_error(Error::new(
                format!("Syntax Error: expected {}", T::desc()),
                parser.peek_span(),
            ));

            Try::Failure
        }
    }
}
