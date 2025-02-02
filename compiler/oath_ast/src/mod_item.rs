use oath_diagnostics::Desc;
use oath_parser::{Parse, Peek, Unmatched};
use oath_tokenizer::keyword;

use crate::{Mod, Pub, Struct, Use};

#[derive(Parse, Peek, Desc)]
#[desc("an item")]
pub enum ModItem {
    Mod(Mod),
    Use(Use),
    Struct(Struct),
    #[dont_peek]
    Unmatched(Unmatched<Self>),
}

#[derive(Parse, Peek, Desc)]
#[desc("an item attrib")]
pub enum ModItemAttrib {
    Pub(Pub),
    Raw(keyword!("raw")),
    Con(keyword!("con")),
    #[dont_peek]
    Unmatched(Unmatched<Self>),
}
