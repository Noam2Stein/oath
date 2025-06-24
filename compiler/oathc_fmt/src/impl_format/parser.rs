use super::*;

impl Format for Leftovers {
    fn format(&self, _interner: &Interner) -> FormatTree {
        FormatTree::AtomString(self.text.clone())
    }
}
