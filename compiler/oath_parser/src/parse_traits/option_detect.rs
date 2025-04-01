use crate::*;

pub trait OptionDetect: Parse {
    fn option_detect(parser: &Parser<impl ParserIterator>) -> bool;
}

impl<T: OptionParse> OptionDetect for Option<T> {
    fn option_detect(parser: &Parser<impl ParserIterator>) -> bool {
        T::detect(parser)
    }
}
