use oath_src::{Position, Span};
use tower_lsp::lsp_types::{self, Range};

#[allow(dead_code)]
#[inline(always)]
pub fn compiler_to_lsp_pos(position: Position) -> lsp_types::Position {
    lsp_types::Position::new(position.line, position.char)
}
#[allow(dead_code)]
#[inline(always)]
pub fn lsp_to_compiler_pos(position: lsp_types::Position) -> Position {
    Position::new(position.line, position.character)
}

#[allow(dead_code)]
#[inline(always)]
pub fn span_to_range(span: Span) -> Range {
    Range::new(
        compiler_to_lsp_pos(span.start()),
        compiler_to_lsp_pos(span.end()),
    )
}
#[allow(dead_code)]
#[inline(always)]
pub fn range_to_span(range: Range) -> Span {
    Span::from_start_end(
        lsp_to_compiler_pos(range.start),
        lsp_to_compiler_pos(range.end),
    )
}
