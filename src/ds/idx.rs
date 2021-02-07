use std::ops::{Deref, DerefMut, Index, IndexMut};

pub struct Idxable<T>(pub Vec<T>);

impl<T> Deref for Idxable<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Idxable<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

macro_rules! impl_index {
    ($($idx:ty),*) => { $(
        impl<T> Index<$idx> for Idxable<T> {
            type Output = T;
            fn index(&self, index: $idx) -> &Self::Output {
                self.0.index(index as usize)
            }
        }
        impl<T> IndexMut<$idx> for Idxable<T> {
            fn index_mut(&mut self, index: $idx) -> &mut Self::Output {
                self.0.index_mut(index as usize)
            }
        }
    )* }
}

impl_index!(i32, i64, i128, isize, u32, u64, u128, usize);
