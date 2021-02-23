pub use crate::trait_alias;

#[macro_export]
macro_rules! trait_alias {
    (pub trait $ident:ident = $($t:tt)*) => {
        pub trait $ident: $($t)* {}
        impl<T: $($t)*> $ident for T {}
    };
}
