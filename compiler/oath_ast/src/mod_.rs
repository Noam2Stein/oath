use oath_parser::Parse;
use oath_tokenizer::{keyword, Ident, SemiPunct};

#[derive(Parse)]
pub struct Mod {
    mod_keyword: keyword!(mod),
    ident: Ident,
    semi: SemiPunct,
}
