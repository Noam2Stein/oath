use oath_parser::{Parse, Peek};
use oath_tokenizer::{keyword, Braces, Group, Ident};

#[derive(Debug, Clone, Parse, Peek)]
pub struct Struct {
    pub struct_keyword: keyword!("struct"),
    pub ident: Ident,
    pub group: Group<Braces>,
}
