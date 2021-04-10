use super::*;
use crate::bits::*;

pub fn bisect<I: Int + Bits>(mut l: I, mut r: I, mut pred: impl FnMut(I) -> bool) -> I {
    while l != r {
        let mid = (l & r) | (l ^ r) >> 1;
        if pred(mid) {
            l = mid + I::ONE;
        } else {
            r = mid;
        }
    }
    r
}
