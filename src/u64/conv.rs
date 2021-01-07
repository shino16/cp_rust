pub use crate::conv::*;
use crate::fp::conv::*;

impl Conv for u64 {
	fn conv_in_place(lhs: &mut Vec<Self>, rhs: &mut Vec<Self>) {
		let r12 = FpC::from(FpB::P).inv();
		let r13 = FpD::from(FpB::P).inv();
		let r23 = FpD::from(FpC::P).inv();
		fn run<M: Mod>(lhs: &mut Vec<u64>, rhs: &mut Vec<u64>) -> Vec<Fp<M>>
		where
			Fp<M>: Conv,
		{
			let lhs = lhs.iter().map(|&e| Fp::from(e)).collect();
			let rhs = rhs.iter().map(|&e| Fp::from(e)).collect();
			Conv::conv(lhs, rhs)
		}
		let v1: Vec<FpB> = run(lhs, rhs);
		let v2: Vec<FpC> = run(lhs, rhs);
		let v3: Vec<FpD> = run(lhs, rhs);
		lhs.resize(v1.len(), Default::default());
		for (((e0, e1), e2), e3) in lhs.iter_mut().zip(v1).zip(v2).zip(v3) {
			let x1 = e1;
			let x2 = (e2 - x1.value()) * r12;
			let x3 = ((e3 - x1.value()) * r13 - x2.value()) * r23;
			let mut x = x1.value() as u64;
			x += x2.value() as u64 * FpB::P as u64;
			x += x3.value() as u64 * FpB::P as u64 * FpC::P as u64;
			*e0 = x;
		}
	}
}
