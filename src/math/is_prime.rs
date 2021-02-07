use crate::u64::*;

/// n < 7e18
pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        false
    } else if n % 6 % 4 != 1 {
        n == 2 || n == 3
    } else {
        let s = (n - 1).trailing_zeros();
        for &a in &[2, 325, 9375, 28178, 450775, 9780504, 1795265022] {
            let mut p = modpow64(a % n, n >> s, n);
            let mut i = s;
            while p != 1 && p != n - 1 && a % n != 0 && i != 0 {
                p = modmul64(p, p, n);
                i -= 1;
            }
            if p != n - 1 && i != s {
                return false;
            }
        }
        true
    }
}
