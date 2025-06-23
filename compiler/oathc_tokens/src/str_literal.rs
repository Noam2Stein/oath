use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, new, Spanned, InternedDisplay)]
#[display("\"{str_id}\"")]
pub struct StrLiteral {
    #[span]
    pub span: Span,
    pub str_id: StrId,
}

const _: () = verify_token_type::<StrLiteral>();

impl Format for StrLiteral {
    fn format(&self, interner: &Interner) -> FormatTree {
        FormatTree::AtomString(self.to_string_interned(interner))
    }
}
