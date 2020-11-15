// verify-helper: PROBLEM https://judge.yosupo.jp/problem/convolution_mod

use lib::io::*;
use lib::fp::*;
use lib::math::ntt::*;

fn main() {
    let mut io = IO::new();
    let (n, m) = io.scan();
    let a = io.scan_vec::<Fp99>(n);
    let b = io.scan_vec::<Fp99>(m);
    if (n, m) == (1, 1) {
        io.println(a[0] * b[0]);
    } else {
        io.iterln(Conv::conv(a, b).into_iter(), " ");
    }
}
