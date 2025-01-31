use oath_parser::Parse;

use crate::ModItem;

#[derive(Parse)]
pub struct ModContent {
    pub items: Vec<ModItem>,
}
