use oath_diagnostics::Desc;
use oath_parser::{Garbage, Parse, Peek};

use crate::Path;

#[derive(Parse, Peek, Desc)]
#[desc("a type")]
pub enum Type {
    Path(Path),
    #[dont_peek]
    Garbage(Garbage<Self>),
}
