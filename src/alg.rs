pub trait Algebraic {
    type Item: Copy;
}

pub trait Monoid: Algebraic {
    fn unit(&self) -> Self::Item;
    fn op(&self, x: Self::Item, y: Self::Item) -> Self::Item;
}

pub trait Group: Monoid {
    fn inv(&self, x: Self::Item) -> Self::Item;
}

pub struct MonoidImpl<T, F>(pub T, pub F);

impl<T: Copy, F> Algebraic for MonoidImpl<T, F> {
    type Item = T;
}

impl<T: Copy, F: Fn(T, T) -> T> Monoid for MonoidImpl<T, F> {
    fn unit(&self) -> Self::Item {
        self.0
    }
    fn op(&self, x: Self::Item, y: Self::Item) -> Self::Item {
        (self.1)(x, y)
    }
}

pub struct GroupImpl<T, F, G>(pub T, pub F, pub G);

impl<T: Copy, F, G> Algebraic for GroupImpl<T, F, G> {
    type Item = T;
}

impl<T: Copy, F: Fn(T, T) -> T, G> Monoid for GroupImpl<T, F, G> {
    fn unit(&self) -> Self::Item {
        self.0
    }
    fn op(&self, x: Self::Item, y: Self::Item) -> Self::Item {
        (self.1)(x, y)
    }
}

impl<T: Copy, F: Fn(T, T) -> T, G: Fn(T) -> T> Group for GroupImpl<T, F, G> {
    fn inv(&self, x: Self::Item) -> Self::Item {
        (self.2)(x)
    }
}
