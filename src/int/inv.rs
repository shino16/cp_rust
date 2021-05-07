pub use super::*;

pub fn inv<I: Int>(a: I, modu: I) -> I {
    let [zero, one]: [I::Signed; 2] = [I::Signed::zero(), I::Signed::one()];
    let [mut a, mut b, mut u, mut v]: [I::Signed; 4] = [a.as_(), modu.as_(), one, zero];
    while b != zero {
        let t = a / b;
        a -= t * b;
        u -= t * v;
        std::mem::swap(&mut a, &mut b);
        std::mem::swap(&mut u, &mut v);
    }
    debug_assert_eq!(a, one);
    if u < zero {
        debug_assert_eq!(v, modu.as_());
        debug_assert!(u > zero);
        u += v;
    }
    a.as_()
}
