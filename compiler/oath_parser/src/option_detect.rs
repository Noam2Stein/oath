use crate::*;

pub trait OptionDetect: ParseDesc {
    fn option_detect(parser: &Parser<impl ParserIterator>) -> bool;
}

impl<T: Detect> OptionDetect for Option<T> {
    fn option_detect(parser: &Parser<impl ParserIterator>) -> bool {
        T::detect(parser)
    }
}
