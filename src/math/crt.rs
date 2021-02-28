use std::u64;

use super::gcd::ext::*;

pub fn crt(rm: &[(u64, u64)]) -> (u64, u64) {
    let (mut r, mut m) = (0, 1);
    for &(ri, mi) in rm {
        let (mut ri, mut mi) = ((ri % mi) as i64, mi as i64);
        if m < mi {
            std::mem::swap(&mut m, &mut mi);
            std::mem::swap(&mut r, &mut ri);
        }
        if m % mi == 0 {
            if r % mi != ri {
                return (0, 0);
            }
        } else {
            let (g, im) = extgcd(m as u64, mi as u64);
            let (g, im) = (g as i64, im as i64);
            if (ri - r) % g != 0 {
                return (0, 0);
            }
            let u = mi / g;
            r += (ri - r) / g % u * im % u * m;
            m *= u;
            if r < 0 {
                r += m;
            }
        }
    }
    (r as u64, m as u64)
}
