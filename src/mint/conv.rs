pub use crate::conv::*;
pub use crate::ds::uvec::*;
pub use crate::mint::*;

macro_rules! impl_ntt {
    ($module:ident, $modu:ty, $prim:expr) => {
        pub mod $module {
            use super::*;

            type FpType = Mint<$modu>;

            static mut ROOT: UVec<FpType> = UVec(Vec::new());
            static mut INV_ROOT: UVec<FpType> = UVec(Vec::new());

            // reserve for n up to 2^k
            pub fn reserve(k: usize) {
                unsafe {
                    if k <= ROOT.len() { return; }
                    ROOT.resize(k, Default::default());
                    INV_ROOT.resize(k, Default::default());
                    let m = FpType::P - 1;
                    let proot = FpType::from($prim);
                    for i in 0..k {
                        ROOT[i] = -proot.pow(m >> (i + 2));
                        INV_ROOT[i] = ROOT[i].inv();
                    }
                }
            }

            pub fn ntt(a: &mut UVec<FpType>) {
                let n = a.len();
                let mut m = n >> 1;
                while m != 0 {
                    let mut w = FpType::ONE;
                    for (k, t) in (0..n).step_by(m * 2).zip(1_u32..) {
                        for i in 0..m {
                            let u = a[k + i];
                            let v = a[k + i + m] * w;
                            a[k + i] = u + v;
                            a[k + i + m] = u - v;
                        }
                        w *= unsafe { ROOT[t.trailing_zeros() as usize] };
                    }
                    m >>= 1;
                }
            }

            pub fn inv_ntt(a: &mut UVec<FpType>) {
                let n = a.len();
                let mut m = 1;
                while m < n {
                    let mut w = FpType::ONE;
                    for (k, t) in (0..n).step_by(m * 2).zip(1_u32..) {
                        for i in 0..m {
                            let u = a[k + i];
                            let v = a[k + i + m];
                            a[k + i] = u + v;
                            a[k + i + m] = (u - v) * w;
                        }
                        w *= unsafe { INV_ROOT[t.trailing_zeros() as usize] };
                    }
                    m <<= 1;
                }
                let d = FpType::from(n).inv();
                a.iter_mut().for_each(|e| *e *= d);
            }

            pub fn conv<'a, 'b>(a: &'a mut UVec<FpType>, b: &'b mut UVec<FpType>) {
                let len = a.len() + b.len() - 1;
                fn ilog2(n: usize) -> u32 {
                    std::mem::size_of::<usize>() as u32 * 8 - n.leading_zeros() - 1
                }
                let n: usize = 1 << ilog2(len * 2 - 1);
                reserve(n.trailing_zeros() as usize);
                a.resize(n, Default::default());
                b.resize(n, Default::default());
                ntt(a);
                ntt(b);
                a.iter_mut().zip(b.iter()).for_each(|(a, b)| *a *= *b);
                b.clear();
                inv_ntt(a);
                a.truncate(len);
            }

            impl Conv for FpType {
                fn conv(mut lhs: Vec<Self>, mut rhs: Vec<Self>) -> Vec<Self> {
                    conv(lhs.as_mut(), rhs.as_mut());
                    lhs
                }
                fn conv_in_place(lhs: &mut Vec<Self>, rhs: &mut Vec<Self>) {
                    conv(lhs.as_mut(), rhs.as_mut());
                }
            }
        }
    };
}

impl_ntt!(impl_b, ModB, 3);
impl_ntt!(impl_c, ModC, 5);
impl_ntt!(impl_d, ModD, 5);

impl Conv for Mint17 {
	fn conv_in_place(lhs: &mut Vec<Self>, rhs: &mut Vec<Self>) {
		let r12 = MintC::from(MintB::P).inv();
		let r13 = MintD::from(MintB::P).inv();
		let r23 = MintD::from(MintC::P).inv();
		fn run<M: Mod>(lhs: &mut Vec<Mint17>, rhs: &mut Vec<Mint17>) -> Vec<Mint<M>>
		where
			Mint<M>: Conv,
		{
			let lhs = lhs.iter().map(|&e| Mint::from(e.value())).collect();
			let rhs = rhs.iter().map(|&e| Mint::from(e.value())).collect();
			Conv::conv(lhs, rhs)
		}
		let v1: Vec<MintB> = run(lhs, rhs);
		let v2: Vec<MintC> = run(lhs, rhs);
		let v3: Vec<MintD> = run(lhs, rhs);
		lhs.resize(v1.len(), Default::default());
		for (((e0, e1), e2), e3) in lhs.iter_mut().zip(v1).zip(v2).zip(v3) {
			let x1 = e1;
			let x2 = (e2 - x1.value()) * r12;
			let x3 = ((e3 - x1.value()) * r13 - x2.value()) * r23;
			let mut x = MintA::from(x1.value());
			x += MintA::from(x2.value()) * MintB::P;
			x += MintA::from(x3.value()) * MintB::P * MintC::P;
			*e0 = x.value().into();
		}
	}
}

// impl Conv for u64 {
// 	fn conv_in_place(lhs: &mut Vec<Self>, rhs: &mut Vec<Self>) {
// 		let r12 = MintC::from(MintB::P).inv();
// 		let r13 = MintD::from(MintB::P).inv();
// 		let r23 = MintD::from(MintC::P).inv();
// 		fn run<M: Mod>(lhs: &mut Vec<u64>, rhs: &mut Vec<u64>) -> Vec<Mint<M>>
// 		where
// 			Mint<M>: Conv,
// 		{
// 			let lhs = lhs.iter().map(|&e| Mint::from(e)).collect();
// 			let rhs = rhs.iter().map(|&e| Mint::from(e)).collect();
// 			Conv::conv(lhs, rhs)
// 		}
// 		let v1: Vec<MintB> = run(lhs, rhs);
// 		let v2: Vec<MintC> = run(lhs, rhs);
// 		let v3: Vec<MintD> = run(lhs, rhs);
// 		lhs.resize(v1.len(), Default::default());
// 		for (((e0, e1), e2), e3) in lhs.iter_mut().zip(v1).zip(v2).zip(v3) {
// 			let x1 = e1;
// 			let x2 = (e2 - x1.value()) * r12;
// 			let x3 = ((e3 - x1.value()) * r13 - x2.value()) * r23;
// 			let mut x = x1.value() as u64;
// 			x += x2.value() as u64 * MintB::P as u64;
// 			x += x3.value() as u64 * MintB::P as u64 * MintC::P as u64;
// 			*e0 = x;
// 		}
// 	}
// }
