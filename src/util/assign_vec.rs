pub use crate::assign_vec;

#[macro_export]
macro_rules! assign_vec {
    ($var:ident, [ $e:expr; $len:expr ]) => {
        $var.clear();
        $var.extend((0..$len).map(|_| $e.clone()));
    }
}
