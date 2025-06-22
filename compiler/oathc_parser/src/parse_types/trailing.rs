use std::marker::PhantomData;

use super::*;

pub struct Trailing<T: OptionParse, S: OptionParse> {
    values: Vec<T>,
    danny: PhantomData<S>,
}

impl<T: OptionParse, S: OptionParse> OptionParse for Trailing<T, S> {
    fn option_parse(parser: &mut impl Tokenizer, output: &mut Option<Self>) -> ParseExit {
        let mut some = Self::parse_error();
        let exit = Self::parse(parser, &mut some);

        *output = Some(some);

        exit
    }

    fn detect(parser: &impl Tokenizer) -> Detection {
        match T::detect(parser) {
            Detection::Detected => Detection::Detected,
            Detection::NotDetected => Detection::EmptyDetected,
            Detection::EmptyDetected => Detection::EmptyDetected,
        }
    }
}
impl<T: OptionParse, S: OptionParse> Parse for Trailing<T, S> {
    fn parse(parser: &mut impl Tokenizer, output: &mut Self) -> ParseExit {
        loop {
            if T::detect(parser) != Detection::Detected {
                break ParseExit::Complete;
            }

            let mut item = None;
            let item_exit = T::option_parse(parser, &mut item);

            if let Some(item) = item {
                output.values.push(item);

                if item_exit == ParseExit::Cut {
                    return ParseExit::Cut;
                }
            } else {
                return item_exit;
            }

            let mut sep = None;
            let sep_exit = S::option_parse(parser, &mut sep);

            if sep.is_none() || sep_exit == ParseExit::Cut {
                return sep_exit;
            }
        }
    }

    fn parse_error() -> Self {
        Self {
            values: Vec::new(),
            danny: PhantomData,
        }
    }
}

impl<T: OptionParse, S: OptionParse> Into<Vec<T>> for Trailing<T, S> {
    fn into(self) -> Vec<T> {
        self.values
    }
}
