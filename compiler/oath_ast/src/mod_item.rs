use oath_parser::Parse;

use crate::Mod;

#[derive(Parse)]
pub enum ModItem {
    Mod(Mod),
}
