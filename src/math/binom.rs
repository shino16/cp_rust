use crate::num::*;

pub struct Binom<T> {
    pub fact: Vec<T>,
    pub inv_fact: Vec<T>,
}

impl<I: Num + From<u32>> Binom<I> {
    pub fn new(n: u32) -> Self {
        let mut fact = Vec::with_capacity(n as usize + 1);
        let mut inv_fact = Vec::with_capacity(n as usize + 1);
        let n: I = n.into();
        let (mut acc, mut now) = (I::ONE, I::ZERO);
        fact.push(I::ONE);
        while now != n {
            now += I::ONE;
            acc *= now;
            fact.push(acc);
        }
        acc = I::ONE / acc;
        while now != I::ZERO {
            inv_fact.push(acc);
            acc *= now;
            now -= I::ONE;
        }
        inv_fact.push(I::ONE);
        inv_fact.reverse();
        Self { fact, inv_fact }
    }
    pub fn call(&self, n: u32, r: u32) -> I {
        if n < r {
            I::ZERO
        } else {
            self.fact[n as usize] * self.inv_fact[r as usize] * self.inv_fact[(n - r) as usize]
        }
    }
}
