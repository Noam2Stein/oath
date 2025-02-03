use oath_diagnostics::Desc;
use oath_parser::{Parse, Peek, Sep, Garbage};
use oath_tokenizer::{keyword, punct, Ident};

#[derive(Parse, Peek)]
pub struct Path {
    pub segments: Sep<PathSegment, punct!("::")>,
}

#[derive(Parse, Peek, Desc)]
#[desc("a path segment")]
pub enum PathSegment {
    Ident(Ident),
    Package(keyword!("package")),
    Super(keyword!("super")),
    Star(punct!("*")),
    #[dont_peek]
    Unmatched(Garbage<Self>),
}
