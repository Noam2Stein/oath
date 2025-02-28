use crate::*;

macro_rules! impl_peek {
    ($ty:ty => $($variant:tt)*) => {
        impl Peek for $ty {
            fn peek(
                tokens: &mut Parser<impl Iterator<Item = TokenTree>>,
                _context: ContextHandle,
            ) -> bool {
                if let Some($($variant)*) = tokens.peek_next() {
                    true
                } else {
                    false
                }
            }
        }
    };
}

impl_peek!(TokenTree => _);

impl_peek!(Ident => TokenTree::Ident(_));

impl_peek!(Keyword => TokenTree::Keyword(_));
with_token_set!($(
    impl_peek!($keyword_type => TokenTree::Keyword(Keyword { kind: KeywordKind::$keyword_variant, .. }));
)*);

impl_peek!(Punct => TokenTree::Punct(_));
with_token_set!($(
    impl_peek!($punct_type => TokenTree::Punct(Punct { kind: PunctKind::$punct_variant, .. }));
)*);

impl_peek!(Literal => TokenTree::Literal(_));
impl_peek!(IntLiteral => TokenTree::Literal(Literal::Int(_)));
impl_peek!(FloatLiteral => TokenTree::Literal(Literal::Float(_)));
impl_peek!(CharLiteral => TokenTree::Literal(Literal::Char(_)));
impl_peek!(StrLiteral => TokenTree::Literal(Literal::Str(_)));

impl_peek!(Group => TokenTree::Group(_));

with_token_set!($(
    impl Peek for Group<$delim_type> {
        fn peek(
            tokens: &mut Parser<impl Iterator<Item = TokenTree>>,
            _context: ContextHandle,
        ) -> bool {
            if let Some(TokenTree::Group(group)) = tokens.peek_next() {
                group.delimiters.kind == DelimiterKind::$delim_type
            } else {
                false
            }
        }
    }
)*);
