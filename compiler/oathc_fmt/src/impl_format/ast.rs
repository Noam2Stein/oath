use super::*;

impl Format for SyntaxTree {
    fn format(&self, interner: &Interner) -> FormatTree {
        FormatTree::Chain(
            self.items
                .iter()
                .map(|item| item.format(interner))
                .chain([FormatTree::AtomString(self.leftovers.text.clone())])
                .collect(),
        )
    }
}
