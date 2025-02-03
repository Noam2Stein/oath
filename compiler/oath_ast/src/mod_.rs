use oath_diagnostics::Desc;
use oath_parser::{Garbage, InBraces, Parse, Peek, RepEndless};
use oath_tokenizer::{keyword, punct, Ident};

use crate::{Fn, Pub, Struct, Use};

#[derive(Parse, Peek)]
pub struct Mod {
    pub mod_keyword: keyword!("mod"),
    pub ident: Ident,
    pub content: ModContent,
}

#[derive(Parse)]
pub enum ModContent {
    Braces(InBraces<Option<RepEndless<ModItem>>>),
    Semi(punct!(";")),
}

#[derive(Parse)]
pub struct ModItem {
    pub attribs: Vec<ModItemAttrib>,
    pub content: ModItemContent,
}

#[derive(Parse, Peek, Desc)]
#[desc("an item")]
pub enum ModItemContent {
    Mod(Mod),
    Use(Use),
    Struct(Struct),
    Fn(Fn),
    #[dont_peek]
    Garbage(Garbage<Self>),
}

#[derive(Parse, Peek)]
pub enum ModItemAttrib {
    Pub(Pub),
    Raw(keyword!("raw")),
    Con(keyword!("con")),
}
