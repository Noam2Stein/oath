use super::*;

#[derive(Debug)]
pub struct Angles {
    pub open: punct!("<"),
    pub close: Try<MorePunct>,
}

impl ParseFrame for Angles {
    fn option_parse<P, T: Tokenizer>(
        parser: &mut Parser<T>,
        parse_t: impl FnOnce(&mut Parser<T>) -> (P, ParseExit),
        _parse_group: impl FnOnce(&mut Parser<GroupTokenizer>) -> (P, ParseExit),
        output: &mut Option<(Self, P)>,
    ) -> ParseExit {
        let mut open = None;
        <punct!("<")>::option_parse(parser, &mut open);

        let open = match open {
            Some(open) => open,
            None => return ParseExit::Complete,
        };

        let (value, parse_exit) = parse_t(parser);

        match parse_exit {
            ParseExit::Complete => {
                let mut close = Try::parse_error();
                let exit = Try::<punct!(">")>::parse(parser, &mut close);

                *output = Some((Self { open, close }, value));

                exit
            }
            ParseExit::Cut => {
                *output = Some((
                    Self {
                        open,
                        close: Try::parse_error(),
                    },
                    value,
                ));

                ParseExit::Cut
            }
        }
    }

    fn detect(parser: &Parser<impl Tokenizer>) -> Detection {
        <punct!("<")>::detect(parser)
    }
}
