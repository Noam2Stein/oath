use oath_diagnostics::Desc;
use oath_parser::{Parse, Peek, Unmatched};
use oath_tokenizer::keyword;

use crate::{Mod, Pub, Struct, Use};

#[derive(Parse, Peek, Desc)]
#[desc("an item")]
pub enum ModSegment {
    Mod(Mod),
    Use(Use),
    Struct(Struct),
    Pub(Pub),
    Raw(keyword!("raw")),
    Con(keyword!("con")),
    #[dont_peek]
    Unmatched(Unmatched<Self>),
}
