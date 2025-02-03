use oath_src::{Span, Spanned};
use oath_tokenizer::TokenTree;

use crate::Parser;

pub fn parse_garbage<I: Iterator<Item = TokenTree>>(
    parser: &mut Parser<I>,
    mut peek_non_garbage: impl FnMut(&mut Parser<I>) -> bool,
) -> Span {
    let mut span = parser
        .next()
        .map_or(Span::end_of_file(), |token| token.span());

    while parser.is_left() && !peek_non_garbage(parser) {
        span = span.connect(parser.next().unwrap().span());
    }

    span
}
