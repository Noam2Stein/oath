use std::ops::{Add, Sub};

use derive_new::new;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, new)]
pub struct Position {
    pub line: u32,
    pub char: u32,
}
impl Position {
    pub const ZERO: Self = Self { line: 0, char: 0 };
}
impl Add<u32> for Position {
    type Output = Self;
    fn add(self, rhs: u32) -> Self::Output {
        Self {
            line: self.line,
            char: self.char + rhs,
        }
    }
}
impl Sub<u32> for Position {
    type Output = Self;
    fn sub(self, rhs: u32) -> Self::Output {
        Self {
            line: self.line,
            char: self.char - rhs,
        }
    }
}
