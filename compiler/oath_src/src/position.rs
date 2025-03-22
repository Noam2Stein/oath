use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
    pub line: u32,
    pub char: u32,
}
impl Position {
    #[inline(always)]
    pub fn new(line: u32, char: u32) -> Self {
        Self { line, char }
    }
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
