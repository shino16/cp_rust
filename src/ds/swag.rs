pub use crate::alg::*;

pub struct Swag<T: Copy, M: Monoid<T>> {
    front: Vec<(T, T)>,
    front_prod: T,
    back: Vec<T>,
    back_prod: T,
    alg: M,
}

impl<T: Copy, M: Monoid<T>> Swag<T, M> {
    pub fn new(alg: M) -> Self {
        Self {
            front: Vec::new(),
            front_prod: alg.unit(),
            back: Vec::new(),
            back_prod: alg.unit(),
            alg,
        }
    }
    pub fn ask(&mut self) -> T {
        self.ensure_front();
        self.alg.op(self.front_prod, self.back_prod)
    }
    pub fn push(&mut self, elt: T) {
        self.back.push(elt);
        self.alg.op_to(elt, &mut self.back_prod);
    }
    pub fn extend<I: IntoIterator<Item = T>>(&mut self, into_iter: I) {
        let iter = into_iter.into_iter();
        self.back.reserve(iter.size_hint().0);
        for elt in iter {
            self.push(elt);
        }
    }
    pub fn extend_from_slice(&mut self, slice: &[T]) {
        self.back.extend_from_slice(slice);
        for &elt in &self.back[self.back.len() - slice.len()..] {
            self.alg.op_to(elt, &mut self.back_prod);
        }
    }
    pub fn pop(&mut self) -> T {
        self.ensure_front();
        let (elt, prod) = self.front.pop().unwrap();
        self.front_prod = prod;
        elt
    }
    fn ensure_front(&mut self) {
        if !self.front.is_empty() {
            return;
        }
        self.front.reserve(self.back.len());
        let mut prod = self.alg.unit();
        while let Some(elt) = self.back.pop() {
            self.front.push((elt, prod));
            self.alg.op_to(elt, &mut prod);
        }
        self.front_prod = prod;
        self.back_prod = self.alg.unit();
    }
}
