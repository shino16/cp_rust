use super::*;

pub fn floor_sqrt<I: UInt>(n: I) -> I {
    if n == I::zero() {
        I::zero()
    } else {
        let x = n.as_::<f64>().sqrt().round().as_();
        (x + n / x) / (I::one() + I::one())
    }
}

pub fn ceil_sqrt<I: UInt>(n: I) -> I {
    if n == I::zero() {
        I::zero()
    } else {
        floor_sqrt(n - I::one()) + I::one()
    }
}
