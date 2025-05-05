use std::fmt::{self, Formatter};

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, new, Spanned)]
pub struct FloatLiteral {
    #[span]
    pub span: Span,
    pub integral: u128,
    pub leading_zeros: u128,
    pub fractional: u128,
    pub suffix: Option<Ident>,
}

verify_token_type!(FloatLiteral);

impl InternedDisplay for FloatLiteral {
    fn interned_fmt(&self, f: &mut Formatter, interner: &Interner) -> fmt::Result {
        write!(f, "{}", self.integral,)?;

        write!(f, ".{}", "0".repeat(self.leading_zeros as usize),)?;

        write!(f, "{}", self.fractional,)?;

        if let Some(suffix) = self.suffix {
            write!(f, "{}", Interned(&suffix, interner))?;
        };

        Ok(())
    }
}
