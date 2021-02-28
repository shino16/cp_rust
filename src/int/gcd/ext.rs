pub use crate::int::*;

/// (g, x) where g = gcd(a, b), ax = g (mod b), 0 <= x < b/g
pub fn extgcd<I: IInt>(mut a: I, mut b: I) -> (I, I) {
    // A = [a, x, y; b, u, v], k = [-1; a; b], Ak = 0
    let (mut x, mut u) = (I::ONE, I::ZERO);
    while !b.is_zero() {
        let t = a / b;
        a -= t * b;
        x -= t * u;
        std::mem::swap(&mut a, &mut b);
        std::mem::swap(&mut x, &mut u);
    }
    if x < I::ZERO {
        x += u;
    }
    (x, a)
}
