use super::*;

#[derive(Debug, new, Spanned)]
pub struct FloatLiteral {
    #[span]
    pub span: Span,
    pub value_integral: Try<u128>,
    pub value_leading_zeros: u128,
    pub value_fraction: Try<u128>,
    pub suffix: Option<LiteralSuffix>,
}

const _: () = verify_token_type::<FloatLiteral>();
