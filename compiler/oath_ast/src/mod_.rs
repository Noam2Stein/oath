use oath_parser::Parse;
use oath_tokenizer::{keyword, punct, Ident};

#[derive(Parse)]
pub struct Mod {
    pub mod_keyword: keyword!("mod"),
    pub ident: Ident,
    pub semi: punct!(";"),
}
