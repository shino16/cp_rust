use crate::zo::*;
use std::ops::Add;

pub fn cuml<T: Copy>(slice: &[T], zero: T, mut op: impl FnMut(T, T) -> T) -> Vec<T> {
    let mut res = Vec::with_capacity(slice.len() + 1);
    let mut tl = zero;
    res.push(tl);
    for &e in slice {
        tl = op(tl, e);
        res.push(tl);
    }
    res
}

pub fn cumr<T: Copy>(slice: &[T], zero: T, mut op: impl FnMut(T, T) -> T) -> Vec<T> {
    let mut res = Vec::with_capacity(slice.len() + 1);
    let mut tl = zero;
    res.push(tl);
    for &e in slice.iter().rev() {
        tl = op(e, tl);
        res.push(tl);
    }
    res.reverse();
    res
}

pub fn cuml_sum<T: Copy + ZeroOne + Add<Output = T>>(slice: &[T]) -> Vec<T> {
    cuml(slice, T::ZERO, Add::add)
}

pub fn cumr_sum<T: Copy + ZeroOne + Add<Output = T>>(slice: &[T]) -> Vec<T> {
    cumr(slice, T::ZERO, Add::add)
}
