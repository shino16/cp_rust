#[cfg(test)]
mod tests {
    mod fp {
        use crate::fp::*;
        #[test]
        fn test_inv() {
            use crate::rng::*;
            let mut rng = Rng32::new();
            for _ in 0..100 {
                let a = Fp17::new(rng.gen());
                let b = a.inv();
                assert!(a * b == Fp17::ONE, "{} {}", a, b);
            }
        }
    }

    mod num {
        use crate::num::*;
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
            use crate::math::gcd::*;
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
}
