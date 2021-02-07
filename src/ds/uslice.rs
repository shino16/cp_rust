use std::ops::{Deref, DerefMut};

#[repr(transparent)]
pub struct USlice<T>(pub [T]);

impl<T> AsRef<USlice<T>> for [T] {
    fn as_ref(&self) -> &USlice<T> {
        unsafe { &*(self as *const [T] as *const USlice<T>) }
    }
}

impl<T> AsMut<USlice<T>> for [T] {
    fn as_mut(&mut self) -> &mut USlice<T> {
        unsafe { &mut *(self as *mut [T] as *mut USlice<T>) }
    }
}

impl<T> AsRef<[T]> for USlice<T> {
    fn as_ref(&self) -> &[T] {
        &self.0
    }
}

impl<T> AsMut<[T]> for USlice<T> {
    fn as_mut(&mut self) -> &mut [T] {
        &mut self.0
    }
}

impl<T> Deref for USlice<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for USlice<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(not(debug_assertions))]
use std::ops::{Index, IndexMut};

#[cfg(not(debug_assertions))]
impl<T> Index<usize> for USlice<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { self.0.get_unchecked(index) }
    }
}

#[cfg(not(debug_assertions))]
impl<T> IndexMut<usize> for USlice<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { self.0.get_unchecked_mut(index) }
    }
}
