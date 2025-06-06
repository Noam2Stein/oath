use super::*;

#[derive(Debug)]
pub struct Angles {
    pub open: punct!("<"),
    pub close: Try<MorePunct>,
}

impl ParseFrame for Angles {
    fn option_parse<Inner, T: Tokenizer>(
        parser: &mut T,
        output: &mut Option<(Self, Inner)>,
        parse_outside: impl FnOnce(&mut T) -> (Inner, ParseExit),
        _parse_inside: impl FnOnce(&mut GroupTokenizer) -> (Inner, ParseExit),
    ) -> ParseExit {
        let mut open = None;
        <punct!("<")>::option_parse(parser, &mut open);

        let open = match open {
            Some(open) => open,
            None => return ParseExit::Complete,
        };

        let (value, parse_exit) = parse_outside(parser);

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

    fn detect(parser: &impl Tokenizer) -> Detection {
        <punct!("<")>::detect(parser)
    }
}
