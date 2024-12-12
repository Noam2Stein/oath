use super::*;

#[derive(Debug, Clone, Hash, Parse, Peek)]
pub struct Func {
    pub func_token: Keyword!("func"),
    pub ident: Ident,
    pub parens: InParens<TerminatedMaybeTrailing<Arg, Punct!(",")>>,
    pub return_type: Option<ReturnType>,
    pub block: Option<Block>,
}

#[derive(Debug, Clone, Hash, Parse)]
pub struct Arg {
    pub mut_token: Option<Keyword!("mut")>,
    pub ident: Ident,
    pub sep: Punct!(":"),
    pub ty: Type,
}

#[derive(Debug, Clone, Hash, Parse, Peek)]
pub struct ReturnType {
    pub arrow: Punct!("->"),
    pub ty: Type,
}
