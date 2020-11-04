pub trait AsInt<T> {
    fn as_(self) -> T;
}

macro_rules! impl_asint {
    ({ $t:ty } => { $($us:ty),* }) => { $(
        impl AsInt<$us> for $t {
            fn as_(self) -> $us {
                self as $us
            }
        }
    )* };
    ({ $t:ty, $($ts:ty),* } => { $($us:ty),* }) => {
        impl_asint!({ $t } => { $($us),* });
        impl_asint!({ $($ts),* } => { $($us),* });
    };
    ($($ts:ty),*) => {
        impl_asint!({ $($ts),* } => { $($ts),* });
    }
}

impl_asint!(i32, i64, i128, isize, u32, u64, u128, usize);
