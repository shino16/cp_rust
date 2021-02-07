pub trait Fill {
    type Item: Clone;
    fn fill(&mut self, val: Self::Item);
}

impl<T: Clone> Fill for [T] {
    type Item = T;
    fn fill(&mut self, val: Self::Item) {
        for e in self {
            *e = val.clone();
        }
    }
}
