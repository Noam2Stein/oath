use crate::*;

pub trait OptionParse: Sized {
    fn option_parse(parser: &mut Parser, output: &mut Option<Self>) -> ParseExit;

    fn detect(parser: &Parser) -> bool;

    fn desc() -> &'static str;
}

impl<T: OptionParse> Parse for Option<T> {
    fn parse(parser: &mut Parser, output: &mut Self) -> ParseExit {
        T::option_parse(parser, output)
    }

    fn parse_error() -> Self {
        None
    }
}
