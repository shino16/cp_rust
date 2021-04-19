pub mod binom;
pub mod bm;
pub mod convex;
pub mod crt;
pub mod factorize;
pub mod gcd;
pub mod is_prime;
pub mod modpow;
pub mod pow;
pub mod primes;

pub fn abs_diff<T: PartialOrd + std::ops::Sub<T, Output = T>>(a: T, b: T) -> T {
    if a < b { b - a } else { a - b }
}
