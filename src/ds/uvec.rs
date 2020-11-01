use std::ops::{Deref, DerefMut};

pub struct UVec<T>(pub Vec<T>);

impl<T> Deref for UVec<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for UVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(not(debug_assertions))]
use std::ops::{Index, IndexMut};

#[cfg(not(debug_assertions))]
impl<T> Index<usize> for UVec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { self.0.get_unchecked(index) }
    }
}

#[cfg(not(debug_assertions))]
impl<T> IndexMut<usize> for UVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { self.0.get_unchecked_mut(index) }
    }
}
