use crate::*;

pub trait ParseDesc: Sized {
    fn desc() -> &'static str;
}
pub trait Parse: ParseDesc {
    fn parse(parser: &mut Parser<impl ParserIterator>) -> Self;
}
pub trait Detect: ParseDesc {
    fn detect(parser: &Parser<impl ParserIterator>) -> bool;
}

impl<T: Parse + Detect> Parse for Option<T> {
    fn parse(parser: &mut Parser<impl ParserIterator>) -> Self {
        if T::detect(parser) {
            Some(T::parse(parser))
        } else {
            None
        }
    }
}
impl<T: ParseDesc> ParseDesc for Option<T> {
    fn desc() -> &'static str {
        T::desc()
    }
}

impl ParseDesc for () {
    fn desc() -> &'static str {
        "nothing"
    }
}
impl Parse for () {
    fn parse(_parser: &mut Parser<impl ParserIterator>) -> Self {
        ()
    }
}

impl<T: ParseDesc> ParseDesc for Box<T> {
    fn desc() -> &'static str {
        T::desc()
    }
}
impl<T: Parse> Parse for Box<T> {
    fn parse(parser: &mut Parser<impl ParserIterator>) -> Self {
        Box::new(T::parse(parser))
    }
}
impl<T: Detect> Detect for Box<T> {
    fn detect(parser: &Parser<impl ParserIterator>) -> bool {
        T::detect(parser)
    }
}

macro_rules! token_impl {
    ($ty:ty = $desc:expr, $variant:tt => $detect:expr) => {
        impl ParseDesc for $ty {
            fn desc() -> &'static str {
                $desc
            }
        }

        impl Parse for Option<$ty> {
            fn parse(parser: &mut Parser<impl ParserIterator>) -> Self {
                if <$ty>::detect(parser) {
                    Some(parser.next().unwrap().try_into().unwrap())
                } else {
                    None
                }
            }
        }

        impl Detect for $ty {
            fn detect(parser: &Parser<impl ParserIterator>) -> bool {
                #[allow(unused_parens)]
                if let Some($variant) = parser.peek() {
                    $detect
                } else {
                    false
                }
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
token_impl!(Punct = "a punct", (TokenTree::Punct(_)) => true);
token_impl!(Group = "delimiters", (TokenTree::Group(_)) => true);
with_token_set!($(
    token_impl!(
        $keyword_type = concat!("`", $keyword, "`"),
        (TokenTree::Keyword(keyword)) => keyword.kind == KeywordKind::$keyword_variant
    );
)*);
with_token_set!($(
    token_impl!(
        $punct_type = concat!("`", $punct, "`"),
        (TokenTree::Punct(punct)) => punct.kind == PunctKind::$punct_variant
    );
)*);
with_token_set!($(
    token_impl!(
        Group<$delim_type> = concat!("`", $delim_open, " ", $delim_close, "`"),
        (TokenTree::Group(group)) => group.delimiters.kind == DelimiterKind::$delim_type
    );
)*);
