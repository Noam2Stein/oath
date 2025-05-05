use std::fmt::{self, Formatter};

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, new, Spanned)]
pub struct IntLiteral {
    #[span]
    pub span: Span,
    pub int: u128,
    pub suffix: Option<Ident>,
}

verify_token_type!(IntLiteral);

impl InternedDisplay for IntLiteral {
    fn interned_fmt(&self, f: &mut Formatter, interner: &Interner) -> fmt::Result {
        write!(f, "{}", self.int,)?;

        if let Some(suffix) = self.suffix {
            write!(f, "{}", Interned(&suffix, interner))?;
        };

        Ok(())
    }
}
