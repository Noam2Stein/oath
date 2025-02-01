use oath_parser::{Parse, Peek};
use oath_tokenizer::{keyword, punct};

use crate::Path;

#[derive(Parse, Peek)]
pub struct Use {
    pub use_keyword: keyword!("use"),
    pub path: Path,
    pub semi: punct!(";"),
}
