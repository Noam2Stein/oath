use crate::*;

pub trait OptionParse: Sized {
    fn option_parse(
        parser: &mut Parser<impl ParserIterator>,
        output: &mut Option<Self>,
    ) -> ParseExit;

    fn detect(parser: &Parser<impl ParserIterator>) -> bool;

    fn desc() -> &'static str;

    fn try_parse(parser: &mut Parser<impl ParserIterator>, output: &mut Try<Self>) -> ParseExit {
        let mut option = None;
        let exit = Self::option_parse(parser, &mut option);

        if let Some(option) = option {
            *output = Try::Success(option);

            exit
        } else {
            parser
                .context()
                .push_error(SyntaxError::Expected(parser.peek_span(), Self::desc()));

            *output = Try::Failure;

            ParseExit::Cut
        }
    }
}

impl<T: OptionParse> Parse for Option<T> {
    fn parse(parser: &mut Parser<impl ParserIterator>, output: &mut Self) -> ParseExit {
        T::option_parse(parser, output)
    }

    fn parse_error() -> Self {
        None
    }
}
impl<T: OptionParse> Parse for Try<T> {
    fn parse(parser: &mut Parser<impl ParserIterator>, output: &mut Self) -> ParseExit {
        T::try_parse(parser, output)
    }

    fn parse_error() -> Self {
        Try::Failure
    }
}
