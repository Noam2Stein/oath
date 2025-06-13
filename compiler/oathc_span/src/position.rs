use std::ops::{Add, AddAssign, Sub, SubAssign};

use derive_new::new;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, new)]
pub struct Position {
    pub file: StrId,
    pub line: u32,
    pub char: u32,
}

impl Add<u32> for Position {
    type Output = Self;

    fn add(self, rhs: u32) -> Self::Output {
        Self {
            line: self.line,
            char: self.char + rhs,
            file: self.file,
        }
    }
}
impl Sub<u32> for Position {
    type Output = Self;

    fn sub(self, rhs: u32) -> Self::Output {
        Self {
            line: self.line,
            char: self.char - rhs,
            file: self.file,
        }
    }
}
impl AddAssign<u32> for Position {
    fn add_assign(&mut self, rhs: u32) {
        *self = *self + rhs;
    }
}
impl SubAssign<u32> for Position {
    fn sub_assign(&mut self, rhs: u32) {
        *self = *self - rhs;
    }
}
