use oath_parser::{Parse, Peek};
use oath_tokenizer::keyword;

#[derive(Parse, Peek)]
pub struct Pub {
    pub pub_keyword: keyword!("pub"),
}
