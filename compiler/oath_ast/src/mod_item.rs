use oath_diagnostics::Desc;
use oath_parser::{Parse, Peek, Unmatched};

use crate::{Mod, Use};

#[derive(Parse, Peek, Desc)]
#[desc("an item")]
pub enum ModItem {
    Mod(Mod),
    Use(Use),
    #[dont_peek]
    Unmatched(Unmatched<Self>),
}
