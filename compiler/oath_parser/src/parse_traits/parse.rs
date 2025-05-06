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
