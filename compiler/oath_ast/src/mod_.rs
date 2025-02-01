use oath_parser::{Parse, Peek};
use oath_tokenizer::{keyword, punct, Ident};

#[derive(Parse, Peek)]
pub struct Mod {
    pub mod_keyword: keyword!("mod"),
    pub ident: Ident,
    pub semi: punct!(";"),
}
