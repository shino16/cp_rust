type Int = i64;
type UInt = u64;

pub mod ext;

pub fn gcd(a: Int, b: Int) -> Int { ugcd(a.abs() as _, b.abs() as _) as _ }
pub fn lcm(a: Int, b: Int) -> Int { a / gcd(a, b) * b }

pub fn ugcd(mut a: UInt, mut b: UInt) -> UInt {
    if a == 0 {
        return b;
    } else if b == 0 {
        return a;
    }
    let a_shift = a.trailing_zeros();
    a >>= a_shift;
    let b_shift = b.trailing_zeros();
    b >>= b_shift;
    while a != b {
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        b -= a;
        b >>= b.trailing_zeros();
    }
    a << a_shift.min(b_shift)
}
pub fn ulcm(a: UInt, b: UInt) -> UInt { a / ugcd(a, b) * b }
