use crate::*;

pub trait OptionDetect: Parse {
    fn option_detect(parser: &Parser<impl ParserIterator>) -> bool;
}
