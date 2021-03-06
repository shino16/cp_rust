pub use crate::assign_vec;

#[macro_export]
macro_rules! assign_vec {
    ($var:ident, [ $($t:tt)* ]) => {
        $var.clear();

    }
}