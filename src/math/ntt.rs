use crate::fp::*;

macro_rules! impl_ntt {
    ($module:ident, $modu:ty, $log2k:expr, $kth_root:expr, $inv_kth_root:expr) => {
        mod $module {
            use super::super::super::fp::*;

            type FpType = Fp<$modu>;

            // modu = c * 2^log2k + 1
            const LOG2K: u32 = $log2k;
            // 2^log2k-th root of unity (== g^c where g: primitive root)
            const KTH_ROOT: u32 = $kth_root;
            const INV_KTH_ROOT: u32 = $inv_kth_root;

            static mut ROOT: Vec<FpType> = Vec::new(); // [n/2..n): n-th roots
            static mut INV_ROOT: Vec<FpType> = Vec::new();
            static mut REV: Vec<usize> = Vec::new(); // bit reversed

            fn log2(n: usize) -> u32 {
                std::mem::size_of::<usize>() as u32 * 8 - (n - 1).leading_zeros()
            }

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
                    for _ in 0..(LOG2K - k) {
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

            unsafe fn ntt(a: &mut [FpType]) {
                let n = a.len();
                let t = ROOT.len().trailing_zeros() - n.trailing_zeros();
                for i in 0..n {
                    if i < REV[i] >> t {
                        a.swap(i, REV[i] >> t);
                    }
                }

                let mut m = 1;
                while m < n {
                    // merge adjacent subarrays
                    for k in (0..n).step_by(m * 2) {
                        for i in 0..m {
                            // butterfly transform
                            let u = a[k + i];
                            let v = a[k + i + m] * ROOT[m + i];
                            a[k + i] = u + v;
                            a[k + i + m] = u - v;
                        }
                    }
                    m <<= 1;
                }
            }

            pub unsafe fn mul(mut a: Vec<FpType>, mut b: Vec<FpType>) -> Vec<FpType> {
                let len = a.len() + b.len() - 1;
                let n: usize = 1 << log2(len);
                reserve(n.trailing_zeros());
                a.resize(n, Default::default());
                b.resize(n, Default::default());
                ntt(&mut a);
                ntt(&mut b);
                a.iter_mut().zip(b.iter()).for_each(|(a, b)| *a *= *b);
                std::mem::swap(&mut ROOT, &mut INV_ROOT);
                ntt(&mut a);
                std::mem::swap(&mut ROOT, &mut INV_ROOT);
                a.truncate(len);
                let d = FpType::from(1) / FpType::from(n);
                for a in &mut a[..] {
                    *a *= d;
                }
                a
            }
        }

        impl Fp<$modu> {
            pub fn conv(lhs: Vec<Self>, rhs: Vec<Self>) -> Vec<Self> {
                unsafe { $module::mul(lhs, rhs) }
            }
        }
    };
}

impl_ntt!(impl99, Mod99, 23, 15_311_432, 469_870_224);
