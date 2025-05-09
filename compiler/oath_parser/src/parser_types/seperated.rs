use std::marker::PhantomData;

use nonempty::NonEmpty;

use super::*;

#[derive(Debug, Clone)]
pub struct Seperated<T: ParseDesc, S: OptionParse> {
    pub values: NonEmpty<T>,
    pub danny: PhantomData<S>,
}

impl<T: ParseDesc, S: OptionParse> OptionParse for Seperated<T, S> {
    fn option_parse(parser: &mut Parser<impl TokenSource>, output: &mut Option<Self>) -> ParseExit {
        let mut first = None;
        let first_exit = T::option_parse(parser, &mut first);

        let first_value = match first {
            Some(first) => first,
            None => return first_exit,
        };

        *output = Some(Self {
            values: NonEmpty::new(first_value),
            danny: PhantomData,
        });

        let values_output = &mut output.as_mut().unwrap().values;

        loop {
            let mut sep = None;
            let sep_exit = S::option_parse(parser, &mut sep);

            if sep.is_none() || sep_exit == ParseExit::Cut {
                return sep_exit;
            }

            let mut item = Try::Failure;
            let item_exit = Try::<T>::parse(parser, &mut item);

            if let Try::Success(item) = item {
                values_output.push(item);

                if item_exit == ParseExit::Cut {
                    return ParseExit::Cut;
                }
            } else {
                return item_exit;
            }
        }
    }

    fn detect(parser: &Parser<impl TokenSource>) -> Detection {
        T::detect(parser)
    }
}

impl<T: ParseDesc, S: OptionParse> ParseDesc for Seperated<T, S> {
    fn desc() -> &'static str {
        T::desc()
    }
}
