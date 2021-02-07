use crate::ds::bitset::*;

pub fn primes(n: u32) -> impl Iterator<Item = u32> {
    let n = n as usize;
    let mut prime = new_bitset(n + 1);
    prime.negate();
    for p in 2..=n {
        if prime.get_bit(p) {
            for j in ((p * 2)..=n).step_by(p) {
                prime.set_bit(j, false);
            }
        }
    }
    struct Iter(usize, usize, Vec<u32>);
    impl Iterator for Iter {
        type Item = u32;
        fn next(&mut self) -> Option<u32> {
            while self.0 < self.1 {
                self.0 += 1;
                if self.2.get_bit(self.0) {
                    return Some(self.0 as u32);
                }
            }
            None
        }
    }
    Iter(1, n, prime)
}
