pub struct Formatter {
    indent: &'static str,
    indent_level: u32,
    string: String,
}
impl Formatter {
    pub fn new(indent: &'static str, capacity: usize) -> Self {
        Self {
            indent,
            indent_level: 0,
            string: String::with_capacity(capacity),
        }
    }

    pub fn newline(&mut self) {
        self.string.push('\n');
        for _ in 0..self.indent_level {
            self.string.push_str(&self.indent);
        }
    }
    pub fn space(&mut self) {
        self.string.push(' ');
    }
    pub fn in_indent(&mut self, write: impl FnOnce(&mut Self)) {
        self.indent_level += 1;
        write(self);
        self.indent_level -= 1;
    }
}
