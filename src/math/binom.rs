use crate::num::*;
use crate::cast::*;

pub struct Binom<T> {
    pub fact: Vec<T>,
    pub inv_fact: Vec<T>,
}

impl<T: Num + From<usize>> Binom<T> {
    pub fn new<I: PrimInt>(n: I) -> Self {
        let n: usize = n.cast();
        let mut fact = Vec::with_capacity(n + 1);
        let mut inv_fact = Vec::with_capacity(n + 1);
        let n: T = n.into();
        let (mut acc, mut now) = (T::one(), T::zero());
        fact.push(T::one());
        while now != n {
            now += T::one();
            acc *= now;
            fact.push(acc);
        }
        acc = T::one() / acc;
        while now != T::zero() {
            inv_fact.push(acc);
            acc *= now;
            now -= T::one();
        }
        inv_fact.push(T::one());
        inv_fact.reverse();
        Self { fact, inv_fact }
    }
    pub fn call<I: PrimInt>(&self, n: I, r: I) -> T {
        let (n, r): (usize, usize) = (n.cast(), r.cast());
        if n < r {
            T::zero()
        } else {
            self.fact[n] * self.inv_fact[r] * self.inv_fact[n - r]
        }
    }
}
