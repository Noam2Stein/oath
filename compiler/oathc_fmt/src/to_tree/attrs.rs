use super::*;

impl ToFormatTree for Repeated<Attr> {
    fn to_format_tree(&self, interner: &Interner) -> FormatTree {
        FormatTree::LineChain(self.values.iter().map(|attr| attr.to_format_tree(interner)).collect())
    }
}

impl ToFormatTree for Attr {
    fn to_format_tree(&self, interner: &Interner) -> FormatTree {
        FormatTree::Chain([self.hash.to_format_tree(interner), self.body.to_format_tree(interner)].into())
    }
}

impl ToFormatTree for AttrBody {
    fn to_format_tree(&self, interner: &Interner) -> FormatTree {
        let inner = match &self.value {
            Some(AttrInput::Fn(fn_input)) => FormatTree::Chain(
                [
                    self.ident.to_format_tree(interner),
                    FormatTree::DenseDelimsList(
                        '(',
                        fn_input.items.values.iter().map(|arg| arg.to_format_tree(interner)).collect(),
                        ')',
                    ),
                ]
                .into(),
            ),
            Some(AttrInput::Set(set)) => FormatTree::Assign(
                Box::new(self.ident.to_format_tree(interner)),
                Box::new(set.value.to_format_tree(interner)),
            ),
            None => self.ident.to_format_tree(interner),
        };

        FormatTree::DenseDelims('[', Box::new(inner), ']')
    }
}
