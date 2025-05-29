use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, new, Spanned, InternedDisplay)]
#[display(
    "{integral}.{}{fractional}{}",
    "0".repeat(self.leading_zeros as usize),
    self.suffix.map(|ident| ident.to_string_interned(interner)).unwrap_or_default(),
)]
pub struct FloatLiteral {
    #[span]
    pub span: Span,
    pub integral: u128,
    pub leading_zeros: u128,
    pub fractional: u128,
    pub suffix: Option<Ident>,
}

const _: () = verify_token_type::<FloatLiteral>();
