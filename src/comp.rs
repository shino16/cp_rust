use crate::slice::*;
use std::collections::HashMap;
use std::hash::Hash;

pub struct Compress<T: Ord>(Vec<T>);

impl<T: Ord> Compress<T> {
    pub fn new(mut data: Vec<T>) -> Self {
        data.sort_unstable();
        data.dedup();
        Self(data)
    }
    pub fn len(&self) -> usize { self.0.len() }
    pub fn compress(&self, v: &T) -> usize {
        let i = lower_bound(&self.0, v);
        debug_assert!(self.0.get(i) == Some(v));
        i
    }
    pub fn restore(&self, i: usize) -> &T { &self.0[i] }
    pub fn cache_al(&self) -> HashMap<T, usize> where T: Clone + Hash {
        self.0.iter().cloned().zip(0..).collect()
    }
    pub fn into_inner(self) -> Vec<T> { self.0 }
}
