#[inline(always)]
pub fn modpow(e: u64, mut k: u64, modu: u64) -> u64 {
    let (mut e, modu) = (e as u64, modu as u64);
    let mut res = 1;
    while k != 0 {
        if k % 2 != 0 {
            res = res * e % modu;
        }
        e = e * e % modu;
        k /= 2;
    }
    res as u64
}
