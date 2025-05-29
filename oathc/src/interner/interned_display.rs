use std::fmt::{Display, Formatter, Result};

use super::*;

pub use oathc_proc_macros::InternedDisplay;

pub trait InternedDisplay: Sized {
    fn interned_fmt(&self, f: &mut Formatter, interner: &Interner) -> Result;

    fn to_string_interned(&self, interner: &Interner) -> String {
        Interned(self, interner).to_string()
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
