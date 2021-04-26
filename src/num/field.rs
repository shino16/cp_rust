pub use super::*;

/// exact division
pub trait Field: INum {}
impl Field for f64 {}
