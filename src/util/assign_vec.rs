pub trait Assign {
    type Item: Clone;
    fn assign(&mut self, e: Self::Item, len: usize);
}

impl<T: Clone> Assign for Vec<T> {
    type Item = T;
    fn assign(&mut self, e: T, len: usize) {
        self.clear();
        self.extend((0..len).map(|_| e.clone()));
    }
}
