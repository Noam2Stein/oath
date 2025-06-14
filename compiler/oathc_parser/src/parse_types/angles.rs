use super::*;

#[derive(Debug)]
pub struct Angles {
    pub open: punct!("<"),
    pub close: Try<MorePunct>,
}

impl FrameDelimiters for Angles {
    fn option_parse_frame<Inner, T: Tokenizer>(
        parser: &mut T,
        output: &mut Option<(Frame<Self>, Inner)>,
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

                *output = Some((
                    Frame {
                        delims: Self { open, close },
                        leftovers: Leftovers::parse_error(),
                    },
                    value,
                ));

                exit
            }
            ParseExit::Cut => {
                *output = Some((
                    Frame {
                        delims: Self {
                            open,
                            close: Try::parse_error(),
                        },
                        leftovers: Leftovers::parse_error(),
                    },
                    value,
                ));

                ParseExit::Cut
            }
        }
    }

    fn detect_frame(parser: &impl Tokenizer) -> Detection {
        <punct!("<")>::detect(parser)
    }
}
