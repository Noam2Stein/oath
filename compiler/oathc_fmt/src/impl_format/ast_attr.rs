use super::*;

impl Format for Attr {
    fn format(&self, interner: &Interner) -> FormatTree {
        FormatTree::Chain([self.hash.format(interner), self.body.format(interner)].into())
    }
}

impl Format for AttrBody {
    fn format(&self, interner: &Interner) -> FormatTree {
        let inner = match &self.value {
            Some(AttrInput::Fn(input)) => FormatTree::Chain([self.ident.format(interner), input.format(interner)].into()),
            Some(AttrInput::Assign(input)) => FormatTree::SpacedChain(
                FormatSpacing::SpaceOrLineTab,
                [self.ident.format(interner), input.format(interner)].into(),
            ),
            None => self.ident.format(interner),
        };

        FormatTree::DenseDelims(
            self.frame.delims.open_str(),
            Box::new(inner),
            self.frame.leftovers.text.clone(),
            self.frame.delims.close_str(),
        )
    }
}
