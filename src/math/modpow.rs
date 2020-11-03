#[inline(always)]
pub fn modpow(e: u32, k: u32, m: u32) -> u32 {
    let (mut e, mut k, m) = (e as u64, k as u64, m as u64);
    let mut res = 1;
    while k != 0 {
        if k % 2 != 0 {
            res *= e;
            res %= m;
        }
        e *= e;
        e %= m;
        k /= 2;
    }
    res as u32
}
