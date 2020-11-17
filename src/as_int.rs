pub trait CastTo<T> {
    fn cast_to(self) -> T;
}
pub trait CastFrom<T> {
    fn cast_from(src: T) -> Self;
}

impl<T, U: CastTo<T>> CastFrom<U> for T {
    fn cast_from(src: U) -> Self {
        U::cast_to(src)
    }
}

macro_rules! impl_prim {
    ($($ts:ty),*) => {
        impl_asint!({ $($ts),* } => { $($ts),* });
        pub trait CastInt where $(Self: CastTo<$ts>),*, $(Self: CastFrom<$ts>),* {}
        $( impl CastInt for $ts {} )*
    }
}

macro_rules! impl_asint {
    ({ $t:ty } => { $($us:ty),* }) => { $(
        impl CastTo<$us> for $t {
            fn cast_to(self) -> $us {
                self as $us
            }
        }
    )* };
    ({ $t:ty, $($ts:ty),* } => { $($us:ty),* }) => {
        impl_asint!({ $t } => { $($us),* });
        impl_asint!({ $($ts),* } => { $($us),* });
    };
}

impl_prim!(i32, i64, i128, isize, u32, u64, u128, usize);

pub trait As: Sized {
    fn as_<T: CastFrom<Self>>(self) -> T {
        T::cast_from(self)
    }
}

impl<T> As for T {}
