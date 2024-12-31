use std::{
    alloc::{alloc, dealloc, Layout},
    cmp::Ordering,
    hash::{Hash, Hasher},
    ptr::copy_nonoverlapping,
};

pub struct Int {
    len: usize,
    union: IntUnion,
}

union IntUnion {
    inlined: usize,
    boxed: *mut usize,
}

impl Drop for Int {
    fn drop(&mut self) {
        if self.len != 1 {
            unsafe {
                dealloc(
                    self.union.boxed as _,
                    Layout::from_size_align_unchecked(
                        size_of::<usize>() * self.len,
                        align_of::<usize>(),
                    ),
                );
            }
        }
    }
}
impl Clone for Int {
    fn clone(&self) -> Self {
        unsafe {
            if self.len == 1 {
                Self {
                    len: self.len,
                    union: IntUnion {
                        inlined: self.union.inlined,
                    },
                }
            } else {
                let boxed = alloc(Layout::from_size_align_unchecked(
                    size_of::<usize>() * self.len,
                    align_of::<usize>(),
                )) as *mut usize;

                copy_nonoverlapping(self.union.boxed, boxed, self.len);

                Self {
                    len: self.len,
                    union: IntUnion { boxed },
                }
            }
        }
    }
}
impl PartialEq for Int {
    fn eq(&self, other: &Self) -> bool {
        if self.len != other.len {
            false
        } else if self.len == 1 {
            unsafe { self.union.inlined == other.union.inlined }
        } else {
            let mut output = true;
            for i in 0..self.len {
                unsafe {
                    output &= *self.union.boxed.add(i) == *other.union.boxed.add(i);
                }
            }
            output
        }
    }
}
impl Eq for Int {}
impl PartialOrd for Int {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Int {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.len < other.len {
            Ordering::Less
        } else if self.len > other.len {
            Ordering::Greater
        } else if self.len == 1 {
            unsafe { self.union.inlined.cmp(&other.union.inlined) }
        } else {
            for i in 0..self.len {
                unsafe {
                    match (*self.union.boxed.add(i)).cmp(&*other.union.boxed.add(i)) {
                        Ordering::Equal => {}
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Less => return Ordering::Less,
                    }
                }
            }
            Ordering::Equal
        }
    }
}
impl Hash for Int {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if self.len == 1 {
            state.write_usize(unsafe { self.union.inlined });
        } else {
            for i in 0..self.len {
                state.write_usize(unsafe { *self.union.boxed.add(i) });
            }
        }
    }
}
impl Default for Int {
    fn default() -> Self {
        Self {
            len: 1,
            union: IntUnion { inlined: 0 },
        }
    }
}
