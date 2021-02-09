pub use crate::pool;

pub trait Pool<T> {
    fn alloc() -> *mut T;
}

#[macro_export]
macro_rules! pool {
    ($ident:ident : [$ty:ty; $len:expr] $(,)?) => {
        struct $ident;
        impl Pool<$ty> for $ident {
            fn alloc() -> *mut $ty {
                use std::mem::MaybeUninit as MU;
                use std::sync::atomic::{AtomicUsize, Ordering};

                static mut DATA: MU<[MU<$ty>; $len]> = MU::uninit();
                static IDX: AtomicUsize = AtomicUsize::new(0);
                let idx = IDX.fetch_add(1, Ordering::Relaxed);
                let data = unsafe { &mut *DATA.as_mut_ptr() };
                data[idx].as_mut_ptr()
            }
        }
    };
}
