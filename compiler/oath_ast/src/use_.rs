use oath_parser::{InBraces, Parse, Peek, SepEnd, TrlEndless};
use oath_tokenizer::{keyword, punct, Ident};

#[derive(Parse, Peek)]
pub struct Use {
    pub use_keyword: keyword!("use"),
    pub path: UsePath,
    pub semi: punct!(";"),
}

#[derive(Parse)]
pub struct UsePath {
    pub segments: SepEnd<Ident, punct!("::"), UsePathEnding>,
}

#[derive(Parse, Peek)]
pub enum UsePathSegment {
    Ident(Ident),
    Package(keyword!("package")),
    Super(keyword!("super")),
}

#[derive(Parse, Peek)]
pub enum UsePathEnding {
    Ident(Ident),
    Super(keyword!("super")),
    Dot(punct!("*")),
    Braces(InBraces<Option<TrlEndless<UsePath, punct!(",")>>>),
}
