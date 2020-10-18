use crate::slice::*;

pub struct Compressed<T>(Box<[T]>);

impl<T: Ord> Compressed<T> {
    pub fn ask(&self, v: T) -> usize {
        debug_assert!(self.0.binary_search(&v).is_ok());
        self.0.lower_bound(&v)
    }
    pub fn restore(&self, i: usize) -> &T {
        &self.0[i]
    }
}

pub struct Compress<T>(Vec<T>);

impl<T: Ord> Compress<T> {
    pub fn new() -> Self {
        Compress(Vec::new())
    }
    pub fn insert(&mut self, v: T) {
        self.0.push(v);
    }
    pub fn compress(mut self) -> Compressed<T> {
        self.0.sort_unstable();
        self.0.dedup();
        Compressed(self.0.into_boxed_slice())
    }
}
