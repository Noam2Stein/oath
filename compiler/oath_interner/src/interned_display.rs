use std::fmt::{Display, Formatter, Result};

use crate::*;

pub trait InternedDisplay {
    fn interned_fmt(&self, f: &mut Formatter, interner: &Interner) -> Result;
}

#[derive(Debug, Clone, Copy)]
pub struct Interned<'t, 'i, T>(pub &'t T, pub &'i Interner);

impl<'t, 'i, T: InternedDisplay> Display for Interned<'t, 'i, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.0.interned_fmt(f, self.1)
    }
}

impl<T: Display> InternedDisplay for T {
    fn interned_fmt(&self, f: &mut Formatter, _interner: &Interner) -> Result {
        write!(f, "{self}")
    }
}

impl InternedDisplay for StrId {
    fn interned_fmt(&self, f: &mut Formatter, interner: &Interner) -> Result {
        interner.unintern_fmt(*self, f)
    }
}
