use super::*;

#[derive(Debug, Default)]
pub struct Leftovers {
    pub error: Option<DiagnosticHandle>,
    pub text: String,
}

impl Leftovers {
    pub fn collect(parser: &mut impl Tokenizer) -> Self {
        if let Some(first_span) = parser.next().map(|first| first.span()) {
            let mut total_span = first_span;
            while let Some(next) = parser.next() {
                total_span = total_span.connect(next.span());
            }

            let error = Some(parser.diagnostics().push_error(Error::UnexpectedTokens(first_span)));

            let text = total_span.index_src(parser.src()).to_string();

            Self { error, text }
        } else {
            Self::default()
        }
    }
}
