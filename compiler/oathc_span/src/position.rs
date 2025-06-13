use std::{
    cmp::Ordering,
    ops::{Add, AddAssign, Sub, SubAssign},
};

use derive_new::new;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, new)]
pub struct Position {
    pub file: FileId,
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

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.file == other.file {
            Some(match self.line.cmp(&other.line) {
                Ordering::Greater => Ordering::Greater,
                Ordering::Less => Ordering::Less,
                Ordering::Equal => self.char.cmp(&other.char),
            })
        } else {
            None
        }
    }
}
impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.file == other.file {
            match self.line.cmp(&other.line) {
                Ordering::Greater => Ordering::Greater,
                Ordering::Less => Ordering::Less,
                Ordering::Equal => self.char.cmp(&other.char),
            }
        } else {
            panic!("tried to compare positions from different files")
        }
    }
}
