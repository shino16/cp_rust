pub use crate::alg::arith::*;

pub trait Cum {
    type Item: Copy;
    fn cuml<M: Monoid<Self::Item>>(&self, m: M) -> Vec<Self::Item>;
    fn cumr<M: Monoid<Self::Item>>(&self, m: M) -> Vec<Self::Item>;
    fn cuml_sum(&self) -> Vec<Self::Item>
    where
        Self::Item: Num,
    {
        self.cuml(MonoidImpl(|| Self::Item::ZERO, |a, b| a + b))
    }
    fn cumr_sum(&self) -> Vec<Self::Item>
    where
        Self::Item: Num,
    {
        self.cumr(MonoidImpl(|| Self::Item::ZERO, |a, b| a + b))
    }
}

impl<T: Copy> Cum for [T] {
    type Item = T;
    fn cuml<M: Monoid<Self::Item>>(&self, m: M) -> Vec<Self::Item> {
        let mut res = Vec::with_capacity(self.len() + 1);
        let mut tl = m.unit();
        res.push(tl);
        for e in self {
            tl = m.op(tl, *e);
            res.push(tl);
        }
        res
    }

    fn cumr<M: Monoid<Self::Item>>(&self, m: M) -> Vec<Self::Item> {
        let mut res = Vec::with_capacity(self.len() + 1);
        let mut tl = m.unit();
        res.push(tl);
        for e in self.iter().rev() {
            tl = m.op(*e, tl);
            res.push(tl);
        }
        res.reverse();
        res
    }
}
