use crate::ds::idx::*;

pub fn prime_table(upto: u32) -> IdxInt<bool> {
    let mut pr = IdxInt(vec![true; upto as usize + 1]);
    pr[0] = false;
    pr[1] = false;
    for i in 2..=upto {
        if pr[i] {
            for j in (i + i..=upto).step_by(i as usize) {
                pr[j] = false;
            }
        }
    }
    pr
}

pub fn primes(upto: u32) -> Vec<u32> {
    let is_prime = prime_table(upto);
    (2..=upto)
        .filter(|&n| is_prime[n])
        .map(|n| n as u32)
        .collect()
}
