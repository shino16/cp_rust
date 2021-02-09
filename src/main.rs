use lib::ds::fenwick::*;
use lib::stdio::*;

fn main() {
    scan! {
        save to input,
        n in usize, q in usize,
        a in [u32; n],
    }
    let mut fwk = FenwickTree::new(a, GroupImpl(|| 0, |a, b| a ^ b, |a| a));
    let mut out = stdout_buf();
    for _ in 0..q {
        scan! {
            from input,
            (t, x, y) in (u32, usize, usize),
        }
        match t {
            1 => fwk.add(x - 1, y as u32),
            _ => prtln!(out, fwk.ask(x - 1, y), " "),
        }
    }
}
