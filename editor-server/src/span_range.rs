use oath::{Position, Span};
use tower_lsp::lsp_types::{self, Range};

#[inline(always)]
pub fn compiler_to_lsp_pos(position: Position) -> lsp_types::Position {
    lsp_types::Position::new(position.line, position.char)
}

#[inline(always)]
pub fn span_to_range(span: Span) -> Range {
    Range::new(compiler_to_lsp_pos(span.start()), compiler_to_lsp_pos(span.end()))
}
