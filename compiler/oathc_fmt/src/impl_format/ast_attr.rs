use super::*;

impl Format for Attr {
    fn format(&self, interner: &Interner) -> FormatTree {
        FormatTree::Chain([self.hash.format(interner), self.body.format(interner)].into())
    }
}

impl Format for AttrBody {
    fn format(&self, interner: &Interner) -> FormatTree {
        let inner = match self.value {
            Some(AttrInput::Fn(input)) => FormatTree::Chain(self.ident.format(interner), input.format(interner)),
            Some(AttrInput::Assign(input)) => {
                FormatTree::Assign(Box::new(self.ident.format(interner)), Box::new(input.value.format(interner)))
            }
            None => self.ident.format(interner),
        };

        FormatTree::DenseDelims(self.frame.delims.open_str(), Box::new(inner), self.frame.delims.close_str())
    }
}
