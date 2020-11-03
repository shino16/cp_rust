pub trait AsInt<T> {
    fn as_(self) -> T;
}

macro_rules! impl_asint {
    ($t:ty => { $($us:ty),* }) => { $(
        impl AsInt<$us> for $t {
            fn as_(self) -> $us {
                self as $us
            }
        }
    )* };
    ({ $t:ty } => { $($us:ty),* }) => {
        impl_asint!($t => { $($us),* });
    };
    ({ $t:ty, $($ts:ty),* } => { $($us:ty),* }) => {
        impl_asint!($t => { $($us),* });
        impl_asint!({ $($ts),* } => { $($us),* });
    };
    ($($ts:ty),*) => {
        impl_asint!({ $($ts),* } => { $($ts),* });
    }
}

impl_asint!(i32, u32, i64, u64, i128, u128, isize, usize);
