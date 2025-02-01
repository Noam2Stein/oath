use oath_diagnostics::Desc;
use oath_parser::{Parse, Peek, Unmatched};

use crate::Mod;

#[derive(Parse, Peek, Desc)]
#[desc("an item")]
pub enum ModItem {
    Mod(Mod),
    #[dont_peek]
    Unmatched(Unmatched<Self>),
}
