use super::*;

impl<T: OptionParse> OptionParse for Box<T> {
    fn option_parse(parser: &mut impl Tokenizer, output: &mut Option<Self>) -> ParseExit {
        let mut inner = None;

        let exit = T::option_parse(parser, &mut inner);

        if let Some(inner) = inner {
            *output = Some(Box::new(inner));
        }

        exit
    }

    fn detect(parser: &impl Tokenizer) -> Detection {
        T::detect(parser)
    }
}
impl<T: Parse> Parse for Box<T> {
    fn parse(parser: &mut impl Tokenizer, output: &mut Self) -> ParseExit {
        let mut inner = T::parse_error();

        let exit = T::parse(parser, &mut inner);

        *output = Box::new(inner);

        exit
    }

    fn parse_error() -> Self {
        Box::new(T::parse_error())
    }
}
impl<T: ParseDesc> ParseDesc for Box<T> {
    fn desc() -> &'static str {
        T::desc()
    }
}
