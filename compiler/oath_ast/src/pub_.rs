use oath_parser::{Parse, Peek};
use oath_tokenizer::keyword;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Parse, Peek)]
pub struct Pub {
    pub pub_keyword: keyword!("pub"),
}
