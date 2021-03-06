pub fn abs_diff<T: Ord + std::ops::Sub<T, Output = T>>(a: T, b: T) -> T {
    if a > b { a - b } else { b - a }
}
