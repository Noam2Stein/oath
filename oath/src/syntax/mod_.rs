use super::*;

#[derive(Debug, Clone, Hash, Parse, Peek)]
pub struct Mod {
    pub mod_token: Keyword!("mod"),
    pub ident: Ident,
    pub content: Option<InBraces<ModContent>>,
}

#[derive(Debug, Clone, Hash, Parse)]
pub struct ModContent {
    pub items: LineTerminated<Item>,
}
