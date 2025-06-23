use super::*;

#[derive(Debug, new, Spanned, InternedDisplay)]
#[display("{value}{}", suffix.as_ref().map_or(String::new(), |suffix| suffix.to_string_interned(interner)))]
pub struct IntLiteral {
    #[span]
    pub span: Span,
    pub value: Try<u128>,
    pub suffix: Option<LiteralSuffix>,
}

const _: () = verify_token_type::<IntLiteral>();

impl Highlightable for IntLiteral {
    fn highlight(&self, color: HighlightColor, h: &mut Vec<Highlight>) {
        self.span.highlight(color, h);
    }
}

impl Format for IntLiteral {
    fn format(&self, interner: &Interner) -> FormatTree {
        FormatTree::AtomString(self.to_string_interned(interner))
    }
}
