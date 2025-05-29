use std::fmt::{Display, Formatter, Result};

use super::*;

#[derive(Debug, Clone, Copy)]
pub struct Interned<'t, 'i, T>(pub &'t T, pub &'i Interner);

impl<'t, 'i, T: InternedDisplay> Display for Interned<'t, 'i, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.0.interned_fmt(f, self.1)
    }
}
