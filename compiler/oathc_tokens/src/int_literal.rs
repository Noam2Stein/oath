use super::*;

#[derive(Debug, new, Spanned)]
pub struct IntLiteral {
    #[span]
    pub span: Span,
    pub value: Try<u128>,
    pub suffix: Option<LiteralSuffix>,
}

const _: () = verify_token_type::<IntLiteral>();
