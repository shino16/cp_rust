use crate::bits::*;
use crate::zo::*;

pub fn bisect<I>(mut l: I, mut r: I, mut pred: impl FnMut(I) -> bool) -> I
where
    I: Bits + ZeroOne + std::ops::Add<I, Output = I> + std::fmt::Debug,
{
    while l != r {
        let mid = (l & r) + ((l ^ r) >> 1);
        if pred(mid) {
            l = mid + I::ONE;
        } else {
            r = mid;
        }
    }
    r
}
