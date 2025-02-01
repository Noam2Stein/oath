use oath_diagnostics::Desc;
use oath_parser::{Parse, Peek, Seperated, Unmatched};
use oath_tokenizer::{keyword, punct, Ident};

#[derive(Parse, Peek)]
pub struct Path {
    pub segments: Seperated<PathSegment, punct!("::")>,
}

#[derive(Parse, Peek, Desc)]
#[desc("a path segment")]
pub enum PathSegment {
    Ident(Ident),
    Package(keyword!("package")),
    Super(keyword!("super")),
    Star(punct!("*")),
    #[dont_peek]
    Unmatched(Unmatched<Self>),
}
