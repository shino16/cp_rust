pub use crate::alg::*;
use crate::bit::*;

pub struct FenwickTree<T, Alg> {
    data: Vec<T>,
    alg: Alg,
}

// Alg: Commutative
impl<Alg: Monoid> FenwickTree<Alg::Item, Alg> {
    pub fn new(mut data: Vec<Alg::Item>, alg: Alg) -> Self {
        let len = data.len();
        data.insert(0, alg.unit());
        for i in 1..=len {
            if i + i.lsb() <= len {
                data[i + i.lsb()] = alg.op(data[i + i.lsb()], data[i]);
            }
        }
        Self { data, alg }
    }
    pub fn add(&mut self, pos: usize, v: Alg::Item) {
        let mut pos = pos + 1;
        while pos < self.data.len() {
            self.data[pos] = self.alg.op(self.data[pos], v);
            pos += pos.lsb();
        }
    }
    pub fn push(&mut self, v: Alg::Item) {
        self.data.push(self.alg.unit());
        self.add(self.data.len() - 1, v);
    }
    pub fn ask_prefix(&self, mut r: usize) -> Alg::Item {
        let mut res = self.alg.unit();
        while r != 0 {
            res = self.alg.op(self.data[r], res);
            r -= r.lsb();
        }
        res
    }
    // unverified
    pub fn partition_point<F: FnMut(Alg::Item) -> bool>(&self, mut pred: F) -> usize {
        let mut x = 0; // pred(&self.ask_prefix(x)) == true
        let mut w = (self.data.len() - 1).msb();
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
    pub fn lower_bound(&self, v: Alg::Item) -> usize
    where
        Alg::Item: Ord,
    {
        self.partition_point(|x| x < v)
    }
    pub fn upper_bound(&self, v: Alg::Item) -> usize
    where
        Alg::Item: Ord,
    {
        self.partition_point(|x| x <= v)
    }
}

impl<Alg: Group> FenwickTree<Alg::Item, Alg> {
    pub fn ask(&self, l: usize, r: usize) -> Alg::Item {
        self.alg
            .op(self.alg.inv(self.ask_prefix(l)), self.ask_prefix(r))
    }
}
