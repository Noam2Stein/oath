use oath_diagnostics::Desc;
use oath_parser::{Garbage, Parse, Peek, Sep};
use oath_tokenizer::{keyword, punct, PunctType};

use crate::{Expr, Path};

#[derive(Parse, Peek)]
pub struct TraitBounds(pub Sep<Trait, punct!(":")>);

#[derive(Parse, Peek, Desc)]
#[desc("a trait")]
pub enum Trait {
    Valid(keyword!("valid")),
    Eq(CmpTrait<punct!("==")>),
    More(CmpTrait<punct!(">")>),
    Less(CmpTrait<punct!("<")>),
    MoreEq(CmpTrait<punct!(">=")>),
    LessEq(CmpTrait<punct!("<=")>),
    Path(Path),
    #[dont_peek]
    Garbage(Garbage<Self>),
}

#[derive(Parse, Peek)]
pub struct CmpTrait<O: PunctType + Parse + Peek> {
    pub punct: O,
    pub value: Expr,
}
