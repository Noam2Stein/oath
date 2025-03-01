use std::ops::Range;

#[derive(Debug, Clone, Default)]
pub struct CodeFmtConfigBuilder {
    pub indent: Option<String>,
    pub space: Option<String>,
    pub newline: Option<String>,
    pub allowed_empty_lines: Option<Range<u32>>,
    pub list_allowed_empty_lines: Option<Range<u32>>,
    pub multiline_trailing_comma: Option<bool>,
    pub singleline_trailing_comma: Option<bool>,
    pub propagate_multiline: Option<bool>,
}

#[derive(Debug, Clone, Default)]
pub struct CodeFmtConfig {
    pub indent: String,
    pub space: String,
    pub newline: String,
    pub allowed_empty_lines: Range<u32>,
    pub list_allowed_empty_lines: Range<u32>,
    pub multiline_trailing_comma: bool,
    pub singleline_trailing_comma: bool,
    pub propagate_multiline: bool,
}

impl CodeFmtConfigBuilder {
    pub fn build(self) -> CodeFmtConfig {
        let allowed_empty_lines = self.allowed_empty_lines.unwrap_or(0..1);
        let list_allowed_empty_lines = self
            .list_allowed_empty_lines
            .unwrap_or(allowed_empty_lines.start..allowed_empty_lines.start);

        CodeFmtConfig {
            indent: self.indent.unwrap_or_else(|| "\t".to_string()),
            space: self.space.unwrap_or_else(|| " ".to_string()),
            newline: self.newline.unwrap_or_else(|| "\n".to_string()),
            list_allowed_empty_lines,
            allowed_empty_lines,
            multiline_trailing_comma: self.multiline_trailing_comma.unwrap_or(true),
            singleline_trailing_comma: self.singleline_trailing_comma.unwrap_or(false),
            propagate_multiline: self.propagate_multiline.unwrap_or(true),
        }
    }
}
