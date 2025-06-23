use super::*;

#[derive(Debug, Default)]
pub struct Leftovers {
    pub error: Option<DiagnosticHandle>,
    pub text: String,
}

impl Parse for Leftovers {
    fn parse(parser: &mut impl Tokenizer, output: &mut Self) -> ParseExit {
        if let Some(first_span) = parser.next().map(|first| first.span()) {
            let mut total_span = first_span;
            while let Some(next) = parser.next() {
                total_span = total_span.connect(next.span());
            }

            output.error = Some(parser.diagnostics().push_error(Error::UnexpectedTokens(first_span)));

            output.text = total_span.index_src(parser.src()).to_string();
        };

        ParseExit::Complete
    }

    fn parse_error() -> Self {
        Self {
            error: None,
            text: String::new(),
        }
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

impl Format for Leftovers {
    fn format(&self, _interner: &Interner) -> FormatTree {
        FormatTree::AtomString(self.text.clone())
    }
}
