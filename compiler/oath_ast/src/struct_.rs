use oath_parser::{Parse, Peek};
use oath_tokenizer::{keyword, Braces, Group, Ident};

use crate::{Contract, GenericParams};

#[derive(Parse, Peek)]
pub struct Struct {
    pub struct_keyword: keyword!("struct"),
    pub ident: Ident,
    pub generics: Option<GenericParams>,
    pub contract: Contract,
    pub group: Group<Braces>,
}
