pub fn assign_if<T>(v: T, var: &mut T, f: impl Fn(&T, &T) -> bool) -> bool {
    if f(&v, var) {
        *var = v;
        true
    } else {
        false
    }
}

pub fn chmin<T: Ord>(v: T, var: &mut T) -> bool {
    assign_if(v, var, |x, y| x < y)
}

pub fn chmax<T: Ord>(v: T, var: &mut T) -> bool {
    assign_if(v, var, |x, y| x > y)
}
