use super::*;

#[derive(Debug)]
pub struct Frame<D: FrameDelimiters> {
    pub delims: D,
    pub leftovers: Leftovers,
}

pub trait FrameDelimiters: Sized {
    fn option_parse_frame<Inner, T: Tokenizer>(
        parser: &mut T,
        output: &mut Option<(Frame<Self>, Inner)>,
        parse_outside: impl FnOnce(&mut T) -> (Inner, ParseExit),
        parse_inside: impl FnOnce(&mut GroupTokenizer) -> (Inner, ParseExit),
    ) -> ParseExit;

    fn detect_frame(parser: &impl Tokenizer) -> Detection;
}

impl<D: FrameDelimiters> Frame<D> {
    pub fn option_parse_frame<Inner, T: Tokenizer>(
        parser: &mut T,
        output: &mut Option<(Self, Inner)>,
        parse_outside: impl FnOnce(&mut T) -> (Inner, ParseExit),
        parse_inside: impl FnOnce(&mut GroupTokenizer) -> (Inner, ParseExit),
    ) -> ParseExit {
        D::option_parse_frame(parser, output, parse_outside, parse_inside)
    }

    pub fn detect_frame(parser: &impl Tokenizer) -> Detection {
        D::detect_frame(parser)
    }
}
