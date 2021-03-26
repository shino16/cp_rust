#[cfg(test)]
mod tests {
    mod gf {
        use crate::gf::*;
        #[test]
        fn test_pow() {
            use crate::rand::xoshiro256plus::*;
            let mut rng = Xoshiro256plus::new();
            assert_eq!(Gf17::new(2).pow(3), Gf17::new(8));
            for _ in 0..100 {
                let base: Gf17 = rng.next().into();
                let k = rng.next() % 100;
                let p = (0..k).map(|_| base).product::<Gf17>();
                assert_eq!(p, base.pow(k));
            }
        }
        #[test]
        fn test_inv() {
            use crate::rand::xoshiro256plus::*;
            let mut rng = Xoshiro256plus::new();
            for _ in 0..100 {
                let a: Gf17 = rng.next().into();
                let b = a.inv();
                assert!(a * b == Gf17::ONE, "{} {}", a, b);
            }
        }
    }

    mod fp_naive {
        use crate::mint::*;
        #[test]
        fn test_mul() {
            use crate::rand::xoshiro256plus::*;
            let mut rng = Xoshiro256plus::new();
            for _ in 0..100 {
                let a = rng.next() as u32 as u64;
                let b = rng.next() as u32 as u64;
                assert_eq!(Mint17::from(a) * b, Mint17::from(a * b));
            }
        }
        #[test]
        fn test_pow() {
            use crate::rand::xoshiro256plus::*;
            let mut rng = Xoshiro256plus::new();
            for _ in 0..100 {
                let base: Mint17 = rng.next().into();
                let k = rng.next() % 100;
                let p = (0..k).map(|_| base).product::<Mint17>();
                assert_eq!(p, base.pow(k as u64));
            }
        }
        #[test]
        fn test_inv() {
            use crate::rand::xoshiro256plus::*;
            let mut rng = Xoshiro256plus::new();
            for _ in 0..100 {
                let a: Mint17 = rng.next().into();
                let b = a.inv();
                assert!(a * b == Mint17::ONE, "{} * {} = {}", a, b, a * b);
            }
        }
    }

    mod func {
        mod memo {
            use crate::func::memo::*;
            #[test]
            fn test_memo() {
                const MOD: u32 = 1_000_000_007;
                let mut fib = vec![1, 1];
                for i in 2..=1000 {
                    let a = fib[i - 1] + fib[i - 2];
                    fib.push(a % MOD);
                }
                let rhs = memoize(|fib, n| {
                    if n <= 1 {
                        1
                    } else {
                        (fib(n - 1) + fib(n - 2)) % MOD
                    }
                })
                .call(1000);
                assert_eq!(fib[1000], rhs);
            }
        }
    }

    mod iter {
        use crate::iter::prod::*;
        use crate::iter::*;
        #[test]
        fn test() {
            let lhs = (0..3).prod(b"ab".to_vec()).collect_vec();
            let rhs = vec![(0, b'a'), (0, b'b'), (1, b'a'), (1, b'b'), (2, b'a'), (2, b'b')];
            assert_eq!(lhs, rhs);
        }
    }

    mod num {
        use crate::int::*;
        #[test]
        fn types() {
            assert_eq!(<i32 as Int>::Signed::ZERO, 0_i32);
            assert_eq!(<i32 as Int>::Unsigned::ZERO, 0_u32);
            assert_eq!(<u32 as Int>::Signed::ZERO, 0_i32);
            assert_eq!(<u32 as Int>::Unsigned::ZERO, 0_u32);
        }
    }

    mod make_vec {
        use crate::make_vec::*;
        #[test]
        fn test() {
            let v = make_vec((3, (5, 8)), "foo");
            assert_eq!(v, vec![vec![vec!["foo"; 8]; 5]; 3]);
        }
    }

    mod math {
        mod gcd {
            use crate::int::gcd::*;
            #[test]
            fn test_gcd() {
                assert_eq!(gcd(0, 0), 0);
                for a in 0..100 {
                    for b in 1..100 {
                        let g = gcd(a, b);
                        for c in g + 1..g {
                            assert!(a % c != 0 || b % c != 0);
                        }
                        assert_eq!(a % g, 0);
                        assert_eq!(b % g, 0);
                    }
                }
            }
        }
    }

    mod slice {
        mod perm {
            use crate::slice::perm::*;
            #[test]
            fn test_next_permutation() {
                let n = 5;
                let mut a: Vec<_> = (0..n).collect();
                let mut b = a.clone();
                let mut cnt = 0;
                while next_permutation(&mut b) {
                    assert!(a < b);
                    next_permutation(&mut a);
                    cnt += 1;
                }
                assert_eq!(cnt, 5 * 4 * 3 * 2 * 1 - 1);
            }
        }
        mod sa {
            use crate::slice::sa::*;
            use crate::slice::lcp::*;
            #[test]
            fn suffix_array_lcp_test() {
                use crate::rand::xoshiro256plus::*;
                let mut rng = Xoshiro256plus::new();
                let modu = rng.next() % 1000;
                let len = rng.next() as usize % 1000;
                let mut v: Vec<_> = std::iter::repeat_with(|| rng.next() % modu + 1).take(len).collect();
                let sa = suffix_array(&mut v, 0, modu + 1, |x| x as usize);
                v.extend_from_slice(&[0; 3]);
                let mut ans: Vec<_> = (0..=len).collect();
                ans.sort_unstable_by_key(|&i| &v[i..]);
                assert_eq!(sa, ans);
                let lcp = lcp_impl(&v, &sa, |v| v as usize);
                for ((&i, &j), lcp) in sa.iter().skip(1).zip(&sa).zip(lcp) {
                    assert_eq!(v[i..i + lcp], v[j..j + lcp]);
                    if i.max(j) + lcp < len {
                        assert_ne!(v[i..i + lcp + 1], v[j..j + lcp + 1]);
                    }
                }
            }
        }
        mod sort {
            use crate::slice::sort::*;
            #[test]
            fn test_count_sort() {
                use crate::rand::xoshiro256plus::*;
                let mut rng = Xoshiro256plus::new();
                let len = rng.next() as usize % 100;
                let modu = rng.next() % len as u64 + 50;
                let mut a: Vec<_> = std::iter::repeat_with(|| (rng.next() % modu, rng.next()))
                    .take(len)
                    .collect();
                let mut b = Vec::new();
                count_sort(&a, &mut b, modu as usize, |x| x.0 as usize);
                a.sort_by_key(|&x| x.0);
                assert_eq!(a, b);
            }
        }
    }
}
