use crate::slice::*;

pub struct Compressed<T>(Box<[T]>);

impl<T: Ord> Compressed<T> {
    pub fn from(mut data: Vec<T>) -> Self {
        data.sort_unstable();
        data.dedup();
        Self(data.into_boxed_slice())
    }
    pub fn ask(&self, v: T) -> usize {
        debug_assert!(self.0.binary_search(&v).is_ok());
        self.0.lower_bound(&v)
    }
    pub fn restore(&self, i: usize) -> &T {
        &self.0[i]
    }
}
