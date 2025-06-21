#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FormatConfig {
    pub max_width: u32,
}

impl Default for FormatConfig {
    fn default() -> Self {
        Self { max_width: 120 }
    }
}
