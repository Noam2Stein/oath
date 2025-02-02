use oath_parser::{Parse, RepEndless};

use crate::{ModItem, ModItemAttrib};

#[derive(Parse)]
pub struct ModContent {
    pub segments: RepEndless<ModSegment>,
}

#[derive(Parse)]
pub enum ModSegment {
    ItemAttrib(ModItemAttrib),
    Item(ModItem),
}
