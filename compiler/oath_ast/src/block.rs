use oath_parser::{InBraces, Parse, Peek};

#[derive(Parse, Peek)]
pub struct Block(pub InBraces<()>);
