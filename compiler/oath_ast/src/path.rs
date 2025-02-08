use oath_diagnostics::Desc;
use oath_parser::{Parse, Peek, Sep};
use oath_tokenizer::{keyword, punct, Ident};

use crate::GenericArgs;

#[derive(Parse, Peek)]
pub struct Path(pub Sep<PathSegment, punct!("::")>);

#[derive(Parse, Peek, Desc)]
#[desc("a path segment")]
pub struct PathSegment {
    pub item: PathSegmentItem,
    pub generics: Option<GenericArgs>,
}

#[derive(Parse, Peek)]
pub enum PathSegmentItem {
    Ident(Ident),
    Package(keyword!("package")),
    Super(keyword!("super")),
}
