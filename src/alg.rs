pub trait Alg {
    type Item: Clone;
}

pub trait Monoid: Alg {
    fn unit(&self) -> Self::Item;
    fn op(&self, x: &Self::Item, y: &Self::Item) -> Self::Item;
}

pub trait Group: Monoid {
    fn inv(&self, x: &Self::Item) -> Self::Item;
}

pub trait Commutative: Monoid {}

macro_rules! impl_commut {
    ($target:ty, $($params:tt : $bounds:tt),*) => {
        impl<$($params: $bounds),*> Commutative for $target {}
    };
}

macro_rules! impl_monoid {
    ($target:ty, $($params:tt : $bounds:tt),*) => {
        impl<$($params : $bounds),*> Alg for $target {
            type Item = T;
        }
        impl<$($params : $bounds),*> Monoid for $target {
            fn unit(&self) -> Self::Item {
                (self.0)()
            }
            fn op(&self, x: &Self::Item, y: &Self::Item) -> Self::Item {
                (self.1)(x, y)
            }
        }
    };
    (commut, $target:ty, $($params:tt : $bounds:tt),*) => {
        impl_commut!($target, $($params : $bounds),*);
        impl_monoid!($target, $($params : $bounds),*);
    };
}

macro_rules! impl_group {
    ($target:ty, $($params:tt : $bounds:tt),*) => {
        impl_monoid!($target, $($params : $bounds),*);
        impl<$($params : $bounds),*> Group for $target {
            fn inv(&self, x: &Self::Item) -> Self::Item {
                (self.2)(x)
            }
        }
    };
    (commut, $target:ty, $($params:tt : $bounds:tt),*) => {
        impl_commut!($target, $($params : $bounds),*);
        impl_group!($target, $($params : $bounds),*);
    };
}

pub struct MonoidImpl<Unit, Op>(pub Unit, pub Op);
pub struct CommutMonoidImpl<Unit, Op>(pub Unit, pub Op);
pub struct GroupImpl<Unit, Op, Inv>(pub Unit, pub Op, pub Inv);
pub struct CommutGroupImpl<Unit, Op, Inv>(pub Unit, pub Op, pub Inv);

// help!
impl_monoid!(MonoidImpl<Unit, Op>, T: Clone, Unit: (Fn() -> T), Op: (Fn(&T, &T) -> T));
impl_monoid!(commut, CommutMonoidImpl<Unit, Op>,
             T: Clone, Unit: (Fn() -> T), Op: (Fn(&T, &T) -> T));
impl_group!(GroupImpl<Unit, Op, Inv>,
            T: Clone, Unit: (Fn() -> T), Op: (Fn(&T, &T) -> T), Inv: (Fn(&T) -> T));
impl_group!(commut, CommutGroupImpl<Unit, Op, Inv>,
            T: Clone, Unit: (Fn() -> T), Op: (Fn(&T, &T) -> T), Inv: (Fn(&T) -> T));
