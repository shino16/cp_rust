pub use crate::ds::uvec as _;
pub use crate::conv::*;

macro_rules! impl_ntt {
    ($module:ident, $modu:ty, $kth_root:expr, $inv_kth_root:expr) => {
        mod $module {
            use super::super::super::ds::uvec::*;
            use super::super::*;
            use super::Conv;

            type FpType = Fp<$modu>;

            // modu = c * 2^K + 1
            const K: u32 = 20;
            // 2^K-th root of unity (== g^c where g: primitive root)
            const KTH_ROOT: u32 = $kth_root;
            const INV_KTH_ROOT: u32 = $inv_kth_root;

            static mut ROOT: UVec<FpType> = UVec(Vec::new()); // [n/2..n): n-th roots
            static mut INV_ROOT: UVec<FpType> = UVec(Vec::new());
            static mut REV: UVec<usize> = UVec(Vec::new()); // bit reversed

            // reserve for n up to 2^k
            pub fn reserve(k: u32) {
                unsafe {
                    let mut n = 1_usize << k;
                    if n <= ROOT.len() {
                        return;
                    }
                    REV.resize(n, Default::default());
                    for i in 0..n {
                        REV[i] = (REV[i >> 1] >> 1) + ((i & 1) << (k - 1));
                    }

                    ROOT.resize(n, Default::default());
                    INV_ROOT.resize(n, Default::default());
                    let mut w = FpType::from(KTH_ROOT);
                    let mut wi = FpType::from(INV_KTH_ROOT);
                    for _ in 0..(K - k) {
                        w *= w;
                        wi *= wi;
                    }
                    let mut wn = FpType::from(-1) * w;
                    let mut wni = FpType::from(-1) * wi;
                    while n >= 2 {
                        for i in (n / 2..n).rev() {
                            ROOT[i] = wni;
                            INV_ROOT[i] = wn;
                            wn *= w;
                            wni *= wi;
                        }
                        n /= 2;
                        w *= w;
                        wi *= wi;
                        wn = FpType::from(-1) * w;
                        wni = FpType::from(-1) * wi;
                    }
                }
            }

            fn ntt(a: &mut UVec<FpType>) {
                unsafe {
                    let n = a.len();
                    let t = ROOT.len().trailing_zeros() - n.trailing_zeros();
                    for i in 0..n {
                        if i < REV[i] >> t {
                            a.swap(i, REV[i] >> t);
                        }
                    }

                    let mut m = 1;
                    while m < n {
                        for k in (0..n).step_by(m * 2) {
                            for i in 0..m {
                                let u = a[k + i];
                                let v = a[k + i + m] * ROOT[m + i];
                                a[k + i] = u + v;
                                a[k + i + m] = u - v;
                            }
                        }
                        m <<= 1;
                    }
                }
            }

            fn mul<'a, 'b>(
                a: &'a mut UVec<FpType>,
                b: &'b mut UVec<FpType>,
            ) -> &'a mut UVec<FpType> {
                let len = a.len() + b.len() - 1;
                let n: usize = len.next_power_of_two();
                reserve(n.trailing_zeros());
                a.resize(n, Default::default());
                b.resize(n, Default::default());
                ntt(a);
                ntt(b);
                a.iter_mut().zip(b.iter()).for_each(|(a, b)| *a *= *b);
                b.clear();
                unsafe {
                    std::mem::swap(&mut ROOT, &mut INV_ROOT);
                }
                ntt(a);
                unsafe {
                    std::mem::swap(&mut ROOT, &mut INV_ROOT);
                }
                a.truncate(len);
                let d = FpType::from(1) / FpType::from(n);
                for a in &mut a[..] {
                    *a *= d;
                }
                a
            }

            impl Conv for FpType {
                fn conv(mut lhs: Vec<Self>, mut rhs: Vec<Self>) -> Vec<Self> {
                    mul(lhs.as_mut(), rhs.as_mut());
                    lhs
                }
                fn conv_in_place(lhs: &mut Vec<Self>, rhs: &mut Vec<Self>) {
                    mul(lhs.as_mut(), rhs.as_mut());
                }
            }
        }
    };
}

impl_ntt!(impl_b, ModB, 565_042_129, 950_391_366);
impl_ntt!(impl_c, ModC, 547_381_916, 603_595_182);
impl_ntt!(impl_d, ModD, 121_832_176, 323_052_423);
