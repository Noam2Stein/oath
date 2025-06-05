use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, new, Spanned, InternedDisplay)]
#[display("\"{str_id}\"")]
pub struct StrLiteral {
    #[span]
    pub span: Span,
    pub str_id: StrId,
}

const _: () = verify_token_type::<StrLiteral>();
