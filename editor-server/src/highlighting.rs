use oathc::*;

use tower_lsp::lsp_types::{SemanticToken, SemanticTokenType};

macro_rules! define_colors {
    ($($color:ident => $lsp_color:expr), * $(,)?) => {
        pub const HIGHLIGHT_LEGEND: &[SemanticTokenType] = &[$($lsp_color), *];

        fn convert_highlight_color(color: HighlightColor) -> u32 {
            match color {$(
                HighlightColor::$color => HIGHLIGHT_LEGEND
                    .iter()
                    .position(|c| c == &$lsp_color)
                    .unwrap() as u32,
            )*}
        }
    };
}
define_colors! {
    Green => SemanticTokenType::TYPE,
    Blue => SemanticTokenType::KEYWORD,
    Cyan => SemanticTokenType::VARIABLE,
    Yellow => SemanticTokenType::FUNCTION,
}

pub fn convert_highlights(highlights: impl Iterator<Item = Highlight>) -> Vec<SemanticToken> {
    let mut highlights = highlights.collect::<Vec<_>>();
    highlights.sort_by(
        |Highlight { span, color: _ },
         Highlight {
             span: other_span,
             color: _,
         }| span.cmp(other_span),
    );

    let mut output = Vec::new();

    let mut prev_line = 0;
    let mut prev_start = 0;

    for Highlight { span, color } in highlights {
        let delta_line = span.line().unwrap_or(0) - prev_line;
        let delta_start = if delta_line == 0 {
            span.start().char - prev_start
        } else {
            span.start().char
        };

        output.push(SemanticToken {
            delta_line: delta_line as u32,
            delta_start: delta_start as u32,
            length: span.len().unwrap_or(1),
            token_type: convert_highlight_color(color),
            token_modifiers_bitset: 0,
        });

        prev_line = span.line().unwrap_or(0);
        prev_start = span.start().char;
    }

    output
}
