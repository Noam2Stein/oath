use std::fmt::{Display, Formatter, Result};

pub trait InternedDisplay {
    fn interned_fmt(&self, f: &mut Formatter) -> Result;
}

impl<T: Display> InternedDisplay for T {
    fn interned_fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{self}")
    }
}
