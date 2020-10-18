pub use crate::alg::*;

pub struct SegmentTree<T, Alg: 'static> {
    len: usize,
    data: Box<[T]>,
    alg: &'static Alg,
}

impl<Alg: Monoid> SegmentTree<Alg::Item, Alg> {
    pub fn new(data: Vec<Alg::Item>, alg: Alg) -> Self {
        let len = data.len();
        let data: Vec<Alg::Item> = (0..len)
            .map(|_| alg.unit())
            .chain(data.into_iter())
            .collect();
        let mut data = data.into_boxed_slice();
        for i in (1..len).rev() {
            data[i] = alg.op(data[i << 1], data[i << 1 | 1]);
        }
        Self { len, data, alg: Box::leak(Box::new(alg)) }
    }
    pub fn add(&mut self, pos: usize, v: Alg::Item) {
        let alg = self.alg;
        self.exec(pos, |x| alg.op(x, v));
    }
    pub fn exec<F: FnOnce(Alg::Item) -> Alg::Item>(&mut self, mut pos: usize, f: F) {
        pos += self.len;
        self.data[pos] = f(self.data[pos]);
        pos >>= 1;
        while pos != 0 {
            self.data[pos] = self.alg.op(self.data[pos << 1], self.data[pos << 1 | 1]);
            pos >>= 1;
        }
    }
    pub fn ask(&self, mut l: usize, mut r: usize) -> Alg::Item {
        let (mut resl, mut resr) = (self.alg.unit(), self.alg.unit());
        l += self.len;
        r += self.len;
        while l < r {
            if l & 1 != 0 {
                resl = self.alg.op(resl, self.data[l]);
                l += 1;
            }
            if r & 1 != 0 {
                resr = self.alg.op(self.data[r - 1], resr);
                r -= 1;
            }
            l >>= 1;
            r >>= 1;
        }
        self.alg.op(resl, resr)
    }
}
