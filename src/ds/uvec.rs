pub use crate::uvec;
use std::ops::{Deref, DerefMut};

#[macro_export]
macro_rules! uvec {
    ($($t:tt)*) => { UVec(vec![$($t)*]) };
}

#[derive(Clone)]
#[repr(transparent)]
pub struct UVec<T>(pub Vec<T>);

impl<T> UVec<T> { pub fn new() -> Self { Self(Vec::new()) } }
impl<T> AsRef<UVec<T>> for Vec<T> {
    fn as_ref(&self) -> &UVec<T> {
        unsafe { &*(self as *const Vec<T> as *const UVec<T>) }
    }
}
impl<T> AsMut<UVec<T>> for Vec<T> {
    fn as_mut(&mut self) -> &mut UVec<T> {
        unsafe { &mut *(self as *mut Vec<T> as *mut UVec<T>) }
    }
}
impl<T> AsRef<Vec<T>> for UVec<T> { fn as_ref(&self) -> &Vec<T> { &self.0 } }
impl<T> AsMut<Vec<T>> for UVec<T> { fn as_mut(&mut self) -> &mut Vec<T> { &mut self.0 } }
impl<T> Deref for UVec<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl<T> DerefMut for UVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

#[cfg(not(debug_assertions))]
mod unchecked {
    use super::*;
    use std::ops::{Index, IndexMut};
    use std::slice::SliceIndex;

    impl<T, I: SliceIndex<[T]>> Index<I> for UVec<T> {
        type Output = I::Output;
        fn index(&self, index: I) -> &Self::Output {
            unsafe { self.0.get_unchecked(index) }
        }
    }
    impl<T, I: SliceIndex<[T]>> IndexMut<I> for UVec<T> {
        fn index_mut(&mut self, index: I) -> &mut Self::Output {
            unsafe { self.0.get_unchecked_mut(index) }
        }
    }
}
