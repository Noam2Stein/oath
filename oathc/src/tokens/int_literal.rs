use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, new, Spanned, InternedDisplay)]
#[display(
    "{int}{}",
    self.suffix.map(|ident| ident.to_string_interned(interner)).unwrap_or_default(),
)]
pub struct IntLiteral {
    #[span]
    pub span: Span,
    pub int: u128,
    pub suffix: Option<Ident>,
}

const _: () = verify_token_type::<IntLiteral>();
