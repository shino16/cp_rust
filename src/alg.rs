pub trait Alg {
    type Item: Copy;
}

pub trait Monoid: Alg {
    fn unit(&self) -> Self::Item;
    fn op(&self, x: Self::Item, y: Self::Item) -> Self::Item;
}

pub trait Group: Monoid {
    fn inv(&self, x: Self::Item) -> Self::Item;
}

pub struct MonoidImpl<Unit, Op>(pub Unit, pub Op);

impl<T: Copy, Unit, Op> Alg for MonoidImpl<Unit, Op>
where
    Unit: Fn() -> T,
{
    type Item = T;
}

impl<T: Copy, Unit, Op> Monoid for MonoidImpl<Unit, Op>
where
    Unit: Fn() -> T,
    Op: Fn(T, T) -> T,
{
    fn unit(&self) -> Self::Item {
        (self.0)()
    }
    fn op(&self, x: Self::Item, y: Self::Item) -> Self::Item {
        (self.1)(x, y)
    }
}

pub struct GroupImpl<Unit, Op, Inv>(pub Unit, pub Op, pub Inv);

impl<T: Copy, Unit, Op, Inv> Alg for GroupImpl<Unit, Op, Inv>
where
    Unit: Fn() -> T,
{
    type Item = T;
}

impl<T: Copy, Unit, Op, Inv> Monoid for GroupImpl<Unit, Op, Inv>
where
    Unit: Fn() -> T,
    Op: Fn(T, T) -> T,
{
    fn unit(&self) -> Self::Item {
        (self.0)()
    }
    fn op(&self, x: Self::Item, y: Self::Item) -> Self::Item {
        (self.1)(x, y)
    }
}

impl<T: Copy, Unit, Op, Inv> Group for GroupImpl<Unit, Op, Inv>
where
    Unit: Fn() -> T,
    Op: Fn(T, T) -> T,
    Inv: Fn(T) -> T,
{
    fn inv(&self, x: Self::Item) -> Self::Item {
        (self.2)(x)
    }
}
