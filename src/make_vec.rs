pub trait VecDim<Elem> {
    type Item;
    fn make_vec(self, elem: Elem) -> Vec<Self::Item>;
}

impl<Elem: Clone> VecDim<Elem> for usize {
    type Item = Elem;
    fn make_vec(self, elem: Elem) -> Vec<Self::Item> {
        vec![elem; self]
    }
}

impl<Elem, Dim> VecDim<Elem> for (usize, Dim)
where
    Dim: VecDim<Elem>,
    Dim::Item: Clone,
{
    type Item = Vec<Dim::Item>;
    fn make_vec(self, elem: Elem) -> Vec<Self::Item> {
        vec![self.1.make_vec(elem); self.0]
    }
}

pub fn make_vec<Elem, Dim: VecDim<Elem>>(dim: Dim, elem: Elem) -> Vec<Dim::Item> {
    dim.make_vec(elem)
}
