use super::*;

#[derive(Debug, Clone, Hash, Parse, Peek)]
pub struct Struct {
    pub struct_token: Keyword!("struct"),
    pub ident: Ident,
    pub braces: InBraces<TerminatedMaybeTrailing<Field, Punct!(",")>>,
}

#[derive(Debug, Clone, Hash, Parse)]
pub struct Field {
    pub ident: Ident,
    pub sep: Punct!(":"),
    pub ty: Type,
}
