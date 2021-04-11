pub use crate::alg::arith::*;

#[derive(Clone)]
pub struct FenwickTree<T, M> {
    data: Vec<T>,
    alg: M,
}

impl<T: Copy, M: Monoid<T>> FenwickTree<T, M> {
    pub fn new(mut data: Vec<T>, alg: M) -> Self {
        let len = data.len();
        data.insert(0, alg.unit());
        for i in 1..=len {
            if i + lsb(i) <= len {
                data[i + lsb(i)] = alg.op(data[i + lsb(i)], data[i]);
            }
        }
        Self { data, alg }
    }
    pub fn len(&self) -> usize {
        self.data.len() - 1
    }
    pub fn clear(&mut self) {
        for e in &mut self.data {
            *e = self.alg.unit();
        }
    }
    pub fn add(&mut self, pos: usize, v: T) {
        let mut pos = pos + 1;
        while pos < self.data.len() {
            self.data[pos] = self.alg.op(self.data[pos], v);
            pos += lsb(pos);
        }
    }
    pub fn push(&mut self, v: T) {
        self.data.push(self.alg.unit());
        self.add(self.data.len() - 1, v);
    }
    pub fn ask_prefix(&self, mut r: usize) -> T {
        let mut res = self.alg.unit();
        while r != 0 {
            res = self.alg.op(self.data[r], res);
            r -= lsb(r);
        }
        res
    }
    pub fn partition_point(&self, mut pred: impl FnMut(T) -> bool) -> usize {
        let mut x = 0; // pred(&self.ask_prefix(x)) == true
        let mut w = self.data.len().next_power_of_two() >> 1;
        let mut l = self.alg.unit();
        while w != 0 {
            if x + w < self.data.len() && pred(self.alg.op(l, self.data[x + w])) {
                x += w;
                l = self.alg.op(l, self.data[x + w]);
            }
            w >>= 1;
        }
        x + 1
    }
    pub fn lower_bound(&self, v: T) -> usize
    where
        T: Ord,
    {
        self.partition_point(|x| x < v)
    }
    pub fn upper_bound(&self, v: T) -> usize
    where
        T: Ord,
    {
        self.partition_point(|x| x <= v)
    }
}

impl<T: Copy, M: Group<T>> FenwickTree<T, M> {
    pub fn sub(&mut self, pos: usize, v: T) {
        self.add(pos, self.alg.inv(v));
    }
    pub fn ask(&self, l: usize, r: usize) -> T {
        self.alg.op(self.alg.inv(self.ask_prefix(l)), self.ask_prefix(r))
    }
}

fn lsb(n: usize) -> usize {
    n & (!n + 1)
}
