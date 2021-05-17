pub trait Cast<T> { fn cast(self) -> T; }
pub trait CastFrom<T> { fn cast_from(src: T) -> Self; }

impl<T, U: Cast<T>> CastFrom<U> for T {
    fn cast_from(src: U) -> Self { U::cast(src) }
}

macro_rules! impl_prim {
    ($($ts:ty),*) => {
        impl_asint!({ $($ts),* } => { $($ts),* });
        pub trait PrimInt where $(Self: Cast<$ts>),*, $(Self: CastFrom<$ts>),* {}
        $( impl PrimInt for $ts {} )*
    }
}

macro_rules! impl_asint {
    ({ $t:ty } => { $($us:ty),* }) => { $(
        impl Cast<$us> for $t { fn cast(self) -> $us { self as $us } }
    )* };
    ({ $t:ty, $($ts:ty),* } => { $($us:ty),* }) => {
        impl_asint!({ $t } => { $($us),* });
        impl_asint!({ $($ts),* } => { $($us),* });
    };
}

impl_prim!(i32, i64, i128, isize, u32, u64, u128, usize);
