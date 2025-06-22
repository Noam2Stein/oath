use super::*;

#[derive(Debug, new, Spanned, InternedDisplay)]
#[display(
    "{value_integral}.{}{value_fraction}{}",
    "0".repeat(self.value_leading_zeros as usize),
    suffix.as_ref().map_or(String::new(), |suffix| suffix.to_string_interned(interner))
)]
pub struct FloatLiteral {
    #[span]
    pub span: Span,
    pub value_integral: Try<u128>,
    pub value_leading_zeros: u128,
    pub value_fraction: Try<u128>,
    pub suffix: Option<LiteralSuffix>,
}

const _: () = verify_token_type::<FloatLiteral>();

impl Highlightable for FloatLiteral {
    fn highlight(&self, color: HighlightColor, h: &mut Vec<Highlight>) {
        self.span.highlight(color, h);
    }
}
