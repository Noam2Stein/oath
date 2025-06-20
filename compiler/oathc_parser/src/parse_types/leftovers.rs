use super::*;

#[derive(Debug, Default)]
pub struct Leftovers {
    pub error: Option<DiagnosticHandle>,
}

impl Parse for Leftovers {
    fn parse(parser: &mut impl Tokenizer, output: &mut Self) -> ParseExit {
        output.error = parser
            .next()
            .map(|next| next.span())
            .map(|span| parser.diagnostics().push_error(Error::UnexpectedTokens(span)));

        ParseExit::Complete
    }

    fn parse_error() -> Self {
        Self { error: None }
    }
}
impl OptionParse for Leftovers {
    fn option_parse(parser: &mut impl Tokenizer, output: &mut Option<Self>) -> ParseExit {
        let mut option = Self::parse_error();
        let exit = Self::parse(parser, &mut option);

        *output = Some(option);

        exit
    }

    fn detect(_parser: &impl Tokenizer) -> Detection {
        Detection::Detected
    }
}
