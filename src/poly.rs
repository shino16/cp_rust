pub use crate::conv::*;
pub use crate::num::field::*;

pub trait Poly: Field + Conv {}
impl<T: Field + Conv> Poly for T {}

// requires F(0) != 0
pub fn inv<T: Poly>(f: Vec<T>, need: usize) -> Vec<T> {
    let (mut f2, mut inv2) = (Vec::new(), Vec::new());
    let mut inv = vec![T::ONE / f[0]];
    let mut deg = 1;
    while deg < need {
        deg *= 2;
        f2.clone_from(&f);
        inv2.clone_from(&inv);
        f2.truncate(deg);
        Conv::conv_in_place(&mut f2, &mut inv2);
        f2.truncate(deg);
        for e in &mut f2 {
            *e = -*e;
        }
        f2[0] += T::ONE + T::ONE;
        Conv::conv_in_place(&mut inv, &mut f2);
        inv.truncate(deg);
    }
    inv
}