use std::marker::PhantomData;

use super::*;

#[derive(Debug, Clone, OptionSpanned)]
pub struct Trailing<T: OptionParse, S: OptionParse> {
    #[span]
    #[option_spanned]
    pub values: Vec<T>,
    pub danny: PhantomData<S>,
}

impl<T: OptionParse, S: OptionParse> OptionParse for Trailing<T, S> {
    fn option_parse(parser: &mut Parser<impl InnerTokenizer>, output: &mut Option<Self>) -> ParseExit {
        let mut option = Self::parse_error();
        let exit = Self::parse(parser, &mut option);

        *output = Some(option);

        exit
    }

    fn detect(parser: &Parser<impl InnerTokenizer>) -> Detection {
        match T::detect(parser) {
            Detection::Detected => Detection::Detected,
            Detection::NotDetected => Detection::EmptyDetected,
            Detection::EmptyDetected => Detection::EmptyDetected,
        }
    }
}
impl<T: OptionParse, S: OptionParse> Parse for Trailing<T, S> {
    fn parse(parser: &mut Parser<impl InnerTokenizer>, output: &mut Self) -> ParseExit {
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
