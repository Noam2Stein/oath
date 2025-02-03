use oath_parser::{InParens, Parse, Peek};
use oath_tokenizer::{keyword, Braces, Group, Ident};

use crate::{GenericParams, Pub};

#[derive(Parse, Peek)]
pub struct Fn {
    #[dont_peek]
    pub pub_attrib: Option<Pub>,
    #[dont_peek]
    pub raw_attrib: Option<keyword!("raw")>,
    #[dont_peek]
    pub con_attrib: Option<keyword!("con")>,
    pub fn_keyword: keyword!("fn"),
    pub ident: Ident,
    pub generics: Option<GenericParams>,
    pub params: InParens<()>,
    pub block: Group<Braces>,
}
