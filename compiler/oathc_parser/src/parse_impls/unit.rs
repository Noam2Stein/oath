use super::*;

impl OptionParse for () {
    fn option_parse(_parser: &mut impl Tokenizer, _output: &mut Option<Self>) -> ParseExit {
        ParseExit::Complete
    }

    fn detect(_parser: &impl Tokenizer) -> Detection {
        Detection::EmptyDetected
    }
}
impl Parse for () {
    fn parse(_parser: &mut impl Tokenizer, _output: &mut Self) -> ParseExit {
        ParseExit::Complete
    }

    fn parse_error() -> Self {
        ()
    }
}
