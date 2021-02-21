pub use crate::int::*;

/// (x, y, g) where ax + by = g, x >= 0
pub fn extgcd<I: IInt>(mut a: I, mut b: I) -> (I, I, I) {
    // A = [a, x, y; b, u, v], k = [-1; a0; b0]
    // A'= [a, x, y; 0, u, v] \therefore a0*u + b0*v = 0
    let (mut x, mut y, mut u, mut v) = (I::ONE, I::ZERO, I::ZERO, I::ONE);
    while !b.is_zero() {
        let t = a / b;
        a -= t * b;
        x -= t * u;
        y -= t * v;
        std::mem::swap(&mut a, &mut b);
        std::mem::swap(&mut x, &mut u);
        std::mem::swap(&mut y, &mut v);
    }
    if x < I::ZERO {
        x += u;
        y -= v;
        // debug_assert_eq!(gcd(u, v), I::ONE);
        debug_assert!(x + u >= I::ZERO);
    }
    (x, y, a)
}
