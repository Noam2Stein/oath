use crate::*;

macro_rules! impl_desc {
    ($ty:ty => $desc:expr) => {
        impl Desc for $ty {
            fn desc() -> &'static str {
                $desc
            }
        }
    };
}
impl_desc!(TokenTree => "a token tree");

impl_desc!(Ident => "an ident");

impl_desc!(Keyword => "a keyword");
with_token_set!($(
    impl_desc!($keyword_type => concat!("`", $keyword, "`"));
)*);

impl_desc!(Punct => "a punct");
with_token_set!($(
    impl_desc!($punct_type => concat!("`", $punct, "`"));
)*);

impl_desc!(Literal => "a literal");
impl_desc!(IntLiteral => "an int literal");
impl_desc!(FloatLiteral => "a float literal");
impl_desc!(StrLiteral => "a string literal");
impl_desc!(CharLiteral => "a char literal");

impl_desc!(Group => "delimiters");
with_token_set!($(
    impl_desc!(Group<$delim_type> => concat!("`", $delim_open, " ", $delim_close, "`"));
)*);
