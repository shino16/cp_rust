pub use crate::alg::*;

pub struct Swag<A: Monoid> {
	front: Vec<(A::Item, A::Item)>,
    front_prod: A::Item,
	back: Vec<A::Item>,
	back_prod: A::Item,
	alg: A,
}

impl<A: Monoid> Swag<A> {
	pub fn new(alg: A) -> Self {
		Self {
			front: Vec::new(),
            front_prod: alg.unit(),
			back: Vec::new(),
			back_prod: alg.unit(),
			alg,
		}
	}
    pub fn ask(&mut self) -> A::Item {
        self.ensure_front();
        self.alg.op(self.front_prod, self.back_prod)
    }
    pub fn push(&mut self, elt: A::Item) {
        self.back.push(elt);
        self.alg.op_to(elt, &mut self.back_prod);
    }
    pub fn extend<I: IntoIterator<Item = A::Item>>(&mut self, into_iter: I) {
        let iter = into_iter.into_iter();
        self.back.reserve(iter.size_hint().0);
        for elt in iter {
            self.push(elt);
        }
    }
    pub fn extend_from_slice(&mut self, slice: &[A::Item]) {
        self.back.extend_from_slice(slice);
        for &elt in &self.back[self.back.len() - slice.len()..] {
            self.alg.op_to(elt, &mut self.back_prod);
        }
    }
    pub fn pop(&mut self) -> A::Item {
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
