use super::*;

impl Format for Param {
    fn format(&self, interner: &Interner) -> FormatTree {
        FormatTree::Chain(
            [
                self.pub_.format(interner),
                self.mut_.format(interner),
                self.body.format(interner),
                self.bounds.format(interner),
            ]
            .into(),
        )
    }
}

impl Format for ParamBody {
    fn format(&self, interner: &Interner) -> FormatTree {
        match self {
            Self::Ident(value) => value.format(interner),
            Self::Tuple(value) => value.format(interner),
        }
    }
}

impl<F: FrameDelimiters> Format for FramedParams<>