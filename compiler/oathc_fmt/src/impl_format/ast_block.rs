use super::*;

impl Format for Block {
    fn format(&self, interner: &Interner) -> FormatTree {
        FormatTree::DenseDelims(
            "{",
            Box::new(FormatTree::SpacedChain(
                FormatSpacing::LineOrTwo,
                self.stmts.iter().map(|item| item.format(interner)).collect(),
            )),
            self.frame.leftovers.text.clone(),
            "}",
        )
    }
}

impl Format for Stmt {
    fn format(&self, interner: &Interner) -> FormatTree {
        match self {
            Self::Break(value) => value.format(interner),
            Self::Continue(value) => value.format(interner),
            Self::Eval(value) => value.format(interner),
            Self::Expr(value) => value.format(interner),
            Self::Item(value) => value.format(interner),
            Self::Let(value) => value.format(interner),
            Self::Return(value) => value.format(interner),
        }
    }
}

impl<K: OptionParse + Format> Format for ControlStmt<K> {
    fn format(&self, interner: &Interner) -> FormatTree {
        FormatTree::Chain(
            [
                self.keyword.format(interner),
                self.value.format(interner),
                self.semi.format(interner),
            ]
            .into(),
        )
    }
}

impl Format for ExprStmt {
    fn format(&self, interner: &Interner) -> FormatTree {
        FormatTree::Chain(
            [
                self.expr.format(interner),
                self.set.format(interner),
                self.semi.format(interner),
            ]
            .into(),
        )
    }
}

impl Format for LetStmt {
    fn format(&self, interner: &Interner) -> FormatTree {
        FormatTree::Chain(
            [
                self.keyword.format(interner),
                self.name.format(interner),
                self.value.format(interner),
                self.semi.format(interner),
            ]
            .into(),
        )
    }
}
