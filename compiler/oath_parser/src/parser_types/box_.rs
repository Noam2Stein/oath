use super::*;

impl<T: Parse> Parse for Box<T> {
    fn parse(parser: &mut Parser, output: &mut Self) -> ParseExit {
        let mut inner = T::parse_error();

        let exit = T::parse(parser, &mut inner);

        *output = Box::new(inner);

        exit
    }

    fn parse_error() -> Self {
        Box::new(T::parse_error())
    }
}

impl<T: OptionParse> OptionParse for Box<T> {
    fn option_parse(parser: &mut Parser, output: &mut Option<Self>) -> ParseExit {
        let mut inner = None;

        let exit = T::option_parse(parser, &mut inner);

        if let Some(inner) = inner {
            *output = Some(Box::new(inner));
        }

        exit
    }

    fn detect(parser: &Parser) -> bool {
        T::detect(parser)
    }

    fn desc() -> &'static str {
        T::desc()
    }
}
