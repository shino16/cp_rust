use crate::int::*;
use crate::cast::*;

pub struct Binom<T> {
    pub fact: Vec<T>,
    pub inv_fact: Vec<T>,
}

impl<I: Num + CastFrom<usize>> Binom<I> {
    pub fn new(n: usize) -> Self {
        let mut fact = Vec::with_capacity(n + 1);
        let mut inv_fact = Vec::with_capacity(n + 1);
        let n: I = n.as_();
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
    pub fn binom<J: CastTo<usize>>(&self, n: J, r: J) -> I {
        let [n, r]: [usize; 2] = [n.as_(), r.as_()];
        self.fact[n] * self.inv_fact[r] * self.inv_fact[n - r]
    }
}
