use super::*;

pub trait FormatExt {
    fn format(&self, config: &FormatConfig) -> String;
}

impl<T: AsRef<str>> FormatExt for T {
    fn format(&self, _config: &FormatConfig) -> String {
        let text = self.as_ref();

        text.to_string()
    }
}
