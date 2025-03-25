use crate::*;

pub trait OptionParse: Detect {
    fn option_parse(parser: &mut Parser<impl ParserIterator>) -> Option<Self>;
}

impl<T: ParseDesc> ParseDesc for Option<T> {
    fn desc() -> &'static str {
        T::desc()
    }
}
impl<T: OptionParse> Parse for Option<T> {
    fn parse(parser: &mut Parser<impl ParserIterator>) -> Self {
        T::option_parse(parser)
    }
}

impl<T: Parse + Detect> OptionParse for T {
    fn option_parse(parser: &mut Parser<impl ParserIterator>) -> Option<Self> {
        if T::detect(parser) {
            Some(T::parse(parser))
        } else {
            None
        }
    }
}
