use tower_lsp::lsp_types::{Position as LspPosition, Range as LspRange};

use super::*;

pub fn convert_position(position: Position) -> LspPosition {
    LspPosition::new(position.line, position.char)
}

pub fn convert_span(span: Span) -> LspRange {
    LspRange::new(convert_position(span.start()), convert_position(span.end()))
}
