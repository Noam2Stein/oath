use oath_diagnostics::Desc;
use oath_parser::{Garbage, Parse, Peek};
use oath_tokenizer::Literal;

use crate::Path;

#[derive(Parse, Peek, Desc)]
#[desc("an expr")]
pub enum Expr {
    Path(Path),
    Literal(Literal),
    #[dont_peek]
    Garbage(Garbage<Self>),
}
