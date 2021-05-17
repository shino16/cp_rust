type UInt = u64;

pub fn floor_sqrt(n: UInt) -> UInt {
    if n == 0 {
        0
    } else {
        let x = (n as f64).sqrt().round() as UInt;
        (x + n / x) / 2
    }
}

pub fn ceil_sqrt(n: UInt) -> UInt {
    if n == 0 {
        0
    } else {
        floor_sqrt(n - 1) + 1
    }
}
