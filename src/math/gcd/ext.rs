type Int = u64;
type IInt = i64;

/// (g, x) where g = gcd(a, b), ax = g (mod b), 0 <= x < b/g
pub fn extgcd(a: Int, b: Int) -> (Int, Int) {
    let (mut a, mut b) = (a as IInt, b as IInt);
    let b0 = b;
    // A = [a, x, y; b, u, v], k = [-1; a; b], Ak = 0
    let (mut x, mut u) = (1, 0);
    while b != 0 {
        let t = a / b;
        a -= t * b;
        x -= t * u;
        std::mem::swap(&mut a, &mut b);
        std::mem::swap(&mut x, &mut u);
    }
    if x < 0 {
        x += b0 / a;
    }
    (a as Int, x as Int)
}
