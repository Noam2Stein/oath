use oath_parser::Parse;
use oath_tokenizer::keyword;

#[derive(Parse)]
pub enum Vis {
    Pub(keyword!("pub")),
    Priv,
}
