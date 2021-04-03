pub use super::*;
pub use crate::conv::*;
pub use crate::ds::uvec::*;

macro_rules! impl_ntt {
    ($module:ident, $modu:ty, $prim:expr) => {
        pub mod $module {
            use super::*;

            type Type = Mint<$modu>;

            pub fn ntt(a: &mut UVec<Type>) {
                let n = a.len();
                assert_eq!(n.count_ones(), 1);
                let r = Type::new($prim);
                let roots: Vec<_> = (0..n.trailing_zeros())
                    .map(|i| -r.pow(((Type::M - 1) >> (i + 2)) as u64))
                    .collect();
                let mut m = n >> 1;
                while m != 0 {
                    let mut w = Type::ONE;
                    for (k, t) in (0..n).step_by(m * 2).zip(1_u32..) {
                        for i in k..k + m {
                            let (u, v) = (a[i], a[i + m] * w);
                            a[i] = u + v;
                            a[i + m] = u - v;
                        }
                        w *= roots[t.trailing_zeros() as usize];
                    }
                    m >>= 1;
                }
            }

            pub fn inv_ntt(a: &mut UVec<Type>) {
                let n = a.len();
                assert_eq!(n.count_ones(), 1);
                let r = Type::new($prim);
                let inv_roots: Vec<_> = (0..n.trailing_zeros())
                    .map(|i| -r.pow((Type::M - 1 - ((Type::M - 1) >> (i + 2))) as u64))
                    .collect();
                let mut m = 1;
                while m < n {
                    let mut w = Type::ONE;
                    for (k, t) in (0..n).step_by(m * 2).zip(1_u32..) {
                        for i in k..k + m {
                            let u = a[i];
                            let v = a[i + m];
                            a[i] = u + v;
                            a[i + m] = (u - v) * w;
                        }
                        w *= inv_roots[t.trailing_zeros() as usize];
                    }
                    m <<= 1;
                }
                let d = Type::from(n as u32).inv();
                a.iter_mut().for_each(|e| *e *= d);
            }

            pub fn conv(a: &mut UVec<Type>, b: &mut UVec<Type>) {
                let len = a.len() + b.len() - 1;
                fn ilog2(n: usize) -> u32 {
                    std::mem::size_of::<usize>() as u32 * 8 - n.leading_zeros() - 1
                }
                let n: usize = 1 << ilog2(len * 2 - 1);
                a.resize_with(n, Default::default);
                b.resize_with(n, Default::default);
                ntt(a);
                ntt(b);
                a.iter_mut().zip(b.iter()).for_each(|(a, b)| *a *= *b);
                b.clear();
                inv_ntt(a);
                a.truncate(len);
            }

            impl Conv for Type {
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
        let r12 = MintC::from(MintB::M).inv();
        let r13 = MintD::from(MintB::M).inv();
        let r23 = MintD::from(MintC::M).inv();
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
        lhs.resize_with(v1.len(), Default::default);
        for (((e0, e1), e2), e3) in lhs.iter_mut().zip(v1).zip(v2).zip(v3) {
            let x1 = e1;
            let x2 = (e2 - x1.value()) * r12;
            let x3 = ((e3 - x1.value()) * r13 - x2.value()) * r23;
            let mut x = MintA::from(x1.value());
            x += MintA::from(x2.value()) * MintB::M;
            x += MintA::from(x3.value()) * MintB::M * MintC::M;
            *e0 = x.value().into();
        }
    }
}
