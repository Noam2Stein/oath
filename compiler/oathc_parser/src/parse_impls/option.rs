use super::*;

impl<T: OptionParse> OptionParse for Option<T> {
    fn option_parse(parser: &mut impl Tokenizer, output: &mut Option<Self>) -> ParseExit {
        let mut option = None;
        let exit = T::option_parse(parser, &mut option);

        *output = Some(option);

        exit
    }

    fn detect(parser: &impl Tokenizer) -> Detection {
        match T::detect(parser) {
            Detection::Detected => Detection::Detected,
            Detection::NotDetected => Detection::EmptyDetected,
            Detection::EmptyDetected => Detection::EmptyDetected,
        }
    }
}
impl<T: OptionParse> Parse for Option<T> {
    fn parse(parser: &mut impl Tokenizer, output: &mut Self) -> ParseExit {
        T::option_parse(parser, output)
    }

    fn parse_error() -> Self {
        None
    }
}
