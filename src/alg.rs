// basic algebraic structures

pub mod arith;

pub trait Monoid<T: Copy> {
    fn unit(&self) -> T;
    fn op(&self, x: T, y: T) -> T;
    fn op_to(&self, y: T, x: &mut T) { *x = self.op(*x, y); }
}

pub trait Group<T: Copy>: Monoid<T> {
    fn inv(&self, x: T) -> T;
    fn op_inv_to(&self, y: T, x: &mut T) { *x = self.op(*x, self.inv(y)) }
}

pub struct MonoidImpl<T: Copy, Unit: Fn() -> T, Op: Fn(T, T) -> T>(pub Unit, pub Op);

pub struct GroupImpl<T: Copy, Unit: Fn() -> T, Op: Fn(T, T) -> T, Inv>(pub Unit, pub Op, pub Inv)
where
    Inv: Fn(T) -> T;

impl<T: Copy, Unit: Fn() -> T, Op: Fn(T, T) -> T> Monoid<T> for MonoidImpl<T, Unit, Op> {
    fn unit(&self) -> T { (self.0)() }
    fn op(&self, x: T, y: T) -> T { (self.1)(x, y) }
}

impl<T: Copy, Unit: Fn() -> T, Op: Fn(T, T) -> T, Inv> Monoid<T> for GroupImpl<T, Unit, Op, Inv>
where
    Inv: Fn(T) -> T,
{
    fn unit(&self) -> T { (self.0)() }
    fn op(&self, x: T, y: T) -> T { (self.1)(x, y) }
}

impl<T: Copy, Unit: Fn() -> T, Op: Fn(T, T) -> T, Inv> Group<T> for GroupImpl<T, Unit, Op, Inv>
where
    Inv: Fn(T) -> T,
{
    fn inv(&self, x: T) -> T { (self.2)(x) }
}
