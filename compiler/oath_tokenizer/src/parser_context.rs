use super::*;

pub struct ParserContext<'a> {
    interner: &'a Interner,
    diagnostics: &'a mut Vec<Diagnostic>,
    highlighting: &'a mut Vec<(Span, HighlightColor)>
}
