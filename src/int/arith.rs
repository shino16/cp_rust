use super::*;

pub fn floor_sqrt<I: UInt>(n: I) -> I {
    if n == I::ZERO {
        I::ZERO
    } else {
        let x = n.as_::<f64>().sqrt().round().as_();
        (x + n / x) / (I::ONE + I::ONE)
    }
}

pub fn ceil_sqrt<I: UInt>(n: I) -> I {
    if n == I::ZERO {
        I::ZERO
    } else {
        floor_sqrt(n - I::ONE) + I::ONE
    }
}
