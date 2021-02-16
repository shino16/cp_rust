pub use crate::alg::*;

fn trunc(x: usize) -> usize { x >> x.trailing_zeros() }

#[derive(Clone)]
pub struct LazySegmentTree<T: Copy, A: Copy, MT: Monoid<T>, MA: Monoid<A>, Apply>
where
    Apply: Fn(T, A) -> T,
{
    len: usize,
    log: u32,
    data: Vec<(T, A)>,
    on_alg: MT,
    act_alg: MA,
    apply: Apply,
}

impl<T: Copy, A: Copy, MT: Monoid<T>, MA: Monoid<A>, Apply: Fn(T, A) -> T>
    LazySegmentTree<T, A, MT, MA, Apply>
{
    pub fn new(len: usize, on_alg: MT, act_alg: MA, apply: Apply) -> Self {
        Self {
            len,
            log: len.next_power_of_two().trailing_zeros(),
            data: vec![(on_alg.unit(), act_alg.unit()); len * 2],
            on_alg,
            act_alg,
            apply,
        }
    }
    pub fn from_slice(slice: &[T], on_alg: MT, act_alg: MA, apply: Apply) -> Self {
        let len = slice.len();
        let log = len.next_power_of_two().trailing_zeros();
        let iter = slice.iter().map(|&x| (x, act_alg.unit()));
        let mut data: Vec<_> = iter.clone().chain(iter).collect();
        for i in (1..len).rev() {
            data[i].0 = on_alg.op(data[i << 1].0, data[i << 1 | 1].0);
        }
        Self { len, log, data, on_alg, act_alg, apply }
    }
    pub fn len(&self) -> usize { self.len }
    fn apply(&mut self, p: usize, actor: A) {
        self.data[p].0 = (self.apply)(self.data[p].0, actor);
        self.act_alg.op_to(actor, &mut self.data[p].1);
    }
    fn flush(&mut self, p: usize) {
        for s in (1..=self.log).rev() {
            let p = p >> s;
            self.apply(p << 1, self.data[p].1);
            self.apply(p << 1 | 1, self.data[p].1);
            self.data[p].1 = self.act_alg.unit();
        }
    }
    fn build(&mut self, mut p: usize) {
        p >>= 1;
        while p != 0 {
            self.data[p].0 = self.on_alg.op(self.data[p << 1].0, self.data[p << 1 | 1].0);
            // debug_assert_eq!(self.data[p].1, self.act_alg.unit());
            p >>= 1;
        }
    }
    pub fn ask(&mut self, l: usize, r: usize) -> T {
        self.flush(trunc(l + self.len()));
        self.flush(trunc(r + self.len()) - 1);
        let [mut resl, mut resr] = [self.on_alg.unit(); 2];
        let (mut l, mut r) = (l + self.len(), r + self.len());
        while l < r {
            if l & 1 != 0 {
                resl = self.on_alg.op(resl, self.data[l].0);
                l += 1;
            }
            if r & 1 != 0 {
                r -= 1;
                resr = self.on_alg.op(self.data[r].0, resr);
            }
            l >>= 1;
            r >>= 1;
        }
        self.on_alg.op(resl, resr)
    }
    pub fn exec<F: FnOnce(&mut T)>(&mut self, pos: usize, f: F) {
        self.flush(pos + self.len());
        let p = pos + self.len();
        f(&mut self.data[p].0);
        self.build(trunc(pos + self.len()));
    }
    pub fn act_over(&mut self, l: usize, r: usize, actor: A) {
        self.flush(trunc(l + self.len()));
        self.flush(trunc(r + self.len()) - 1);
        {
            let (mut l, mut r) = (l + self.len(), r + self.len());
            while l < r {
                if l & 1 != 0 {
                    self.apply(l, actor);
                    l += 1;
                }
                if r & 1 != 0 {
                    r -= 1;
                    self.apply(r, actor);
                }
                l >>= 1;
                r >>= 1;
            }
        }
        self.build(trunc(l + self.len()));
        self.build(trunc(r + self.len()) - 1);
    }
    pub fn flush_all(&mut self) -> &[(T, A)] {
        for p in 1..self.len() {
            self.apply(p << 1, self.data[p].1);
            self.apply(p << 1 | 1, self.data[p].1);
            self.data[p].1 = self.act_alg.unit();
        }
        &self.data[self.len()..]
    }
}
