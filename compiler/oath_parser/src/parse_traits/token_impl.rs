use crate::*;

macro_rules! token_impl {
    ($ty:ty = $desc:expr, $variant:tt => $detect:expr) => {
        impl OptionParse for $ty {
            fn option_parse(parser: &mut Parser<impl ParserIterator>) -> Option<Self> {
                Self::detect(parser).then(|| parser.next().unwrap().try_into().unwrap())
            }

            fn detect(parser: &Parser<impl ParserIterator>) -> bool {
                #[allow(unused_parens)]
                if let Some($variant) = parser.peek() {
                    $detect
                } else {
                    false
                }
            }

            fn desc() -> &'static str {
                $desc
            }
        }
    };
}

token_impl!(TokenTree = "a token tree", _ => true);

token_impl!(Ident = "an ident", (TokenTree::Ident(_)) => true);

token_impl!(Literal = "a literal", (TokenTree::Literal(_)) => true);
token_impl!(IntLiteral = "an int literal", (TokenTree::Literal(Literal::Int(_))) => true);
token_impl!(FloatLiteral = "a float literal", (TokenTree::Literal(Literal::Float(_))) => true);
token_impl!(StrLiteral = "a string literal", (TokenTree::Literal(Literal::Str(_))) => true);
token_impl!(CharLiteral = "a char literal", (TokenTree::Literal(Literal::Char(_))) => true);

token_impl!(Keyword = "a keyword", (TokenTree::Keyword(_)) => true);
with_tokens!($(
    token_impl!(
        $keyword_type = concat!("`", $keyword, "`"),
        (TokenTree::Keyword(keyword)) => keyword.kind == KeywordKind::$keyword_variant
    );
)*);

token_impl!(Punct = "a punct", (TokenTree::Punct(_)) => true);
with_tokens!($(
    token_impl!(
        $punct_type = concat!("`", $punct, "`"),
        (TokenTree::Punct(punct)) => punct.kind == PunctKind::$punct_variant
    );
)*);

token_impl!(Group = "delimiters", (TokenTree::Group(_)) => true);
with_tokens!($(
    token_impl!(
        Group<$delim_type> = concat!("`", $delim_open, " ", $delim_close, "`"),
        (TokenTree::Group(group)) => group.delimiters.kind == DelimiterKind::$delim_type
    );
)*);
