use super::*;

pub fn bisect<I: Int, F: FnMut(I) -> bool>(mut l: I, mut r: I, mut pred: F) -> I {
    let two = I::ONE + I::ONE;
    while l != r {
        let mid = (l + r) / two;
        if pred(mid) {
            l = mid + I::ONE;
        } else {
            r = mid;
        }
    }
    r
}
