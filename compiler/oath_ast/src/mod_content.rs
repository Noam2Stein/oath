use oath_parser::{Endless, Parse};

use crate::ModItem;

#[derive(Parse)]
pub struct ModContent {
    pub items: Endless<ModItem>,
}
