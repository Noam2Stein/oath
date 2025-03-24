use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Try<T> {
    Success(T),
    Failure,
}

impl<T: ParseDesc> ParseDesc for Try<T> {
    fn desc() -> &'static str {
        T::desc()
    }
}

impl<T: ParseDesc> Parse for Try<T>
where
    Option<T>: Parse,
{
    fn parse(parser: &mut Parser<impl ParserIterator>) -> Self {
        if let Some(output) = Parse::parse(parser) {
            Self::Success(output)
        } else {
            parser.context().push_error(Error::new(
                format!("Syntax Error: expected {}", T::desc()),
                parser.peek_span(),
            ));

            Self::Failure
        }
    }
}
