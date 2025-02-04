use oath_parser::{InParens, Parse, Peek};
use oath_tokenizer::{keyword, Braces, Group, Ident};

use crate::GenericParams;

#[derive(Parse, Peek)]
pub struct Fn {
    pub fn_keyword: keyword!("fn"),
    pub ident: Ident,
    pub generics: Option<GenericParams>,
    pub params: InParens<()>,
    pub block: Group<Braces>,
}
