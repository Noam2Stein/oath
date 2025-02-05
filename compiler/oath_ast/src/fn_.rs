use oath_parser::{InParens, Parse, Peek, TrlEndless};
use oath_tokenizer::{keyword, punct, Braces, Group, Ident};

use crate::{Contract, GenericParams, TraitBounds, Type};

#[derive(Parse, Peek)]
pub struct Fn {
    pub fn_keyword: keyword!("fn"),
    pub ident: Ident,
    pub generics: Option<GenericParams>,
    pub params: InParens<Option<TrlEndless<FnParam, punct!(",")>>>,
    pub contract: Contract,
    pub block: Group<Braces>,
}

#[derive(Parse)]
pub struct FnParam {
    pub mut_: Option<keyword!("mut")>,
    pub ident: Ident,
    pub sep: punct!(":"),
    pub type_: Type,
    pub bounds: Option<FnParamBounds>,
}

#[derive(Parse, Peek)]
pub struct FnParamBounds {
    pub sep: punct!(":"),
    pub bounds: TraitBounds,
}
