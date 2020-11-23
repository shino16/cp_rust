use std::io::{stdout, BufWriter, Read, StdoutLock, Write};
use std::marker::PhantomData;

pub struct IO {
    iter: std::str::SplitAsciiWhitespace<'static>,
    buf: BufWriter<StdoutLock<'static>>,
}

impl IO {
    pub fn new() -> Self {
        let mut input = String::new();
        std::io::stdin().read_to_string(&mut input).unwrap();
        let input = Box::leak(input.into_boxed_str());
        let out = Box::leak(Box::new(stdout()));
        IO { iter: input.split_ascii_whitespace(), buf: BufWriter::new(out.lock()) }
    }
    fn scan_str(&mut self) -> &'static str { self.iter.next().unwrap() }
    fn scan_raw(&mut self) -> &'static [u8] { self.scan_str().as_bytes() }
    pub fn scan<T: Scan>(&mut self) -> T { T::scan(self) }
    pub fn scan_iter<T: Scan>(&mut self) -> Iter<'_, T> { Iter { io: self, _m: PhantomData } }
    pub fn scan_n<T: Scan>(&mut self, n: usize) -> std::iter::Take<Iter<'_, T>> {
        self.scan_iter().take(n)
    }
    pub fn scan_vec<T: Scan>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.scan()).collect()
    }

    pub fn print<T: Print>(&mut self, x: T) { T::print(self, x); }
    pub fn println<T: Print>(&mut self, x: T) {
        self.print(x);
        self.print("\n");
    }
    pub fn iterln<T: Print, I: IntoIterator<Item = T>>(&mut self, into_iter: I, delim: &str) {
        let mut iter = into_iter.into_iter();
        if let Some(v) = iter.next() {
            self.print(v);
            for v in iter {
                self.print(delim);
                self.print(v);
            }
        }
        self.print("\n");
    }
    pub fn flush(&mut self) { self.buf.flush().unwrap(); }
}

pub struct Iter<'a, T> {
    io: &'a mut IO,
    _m: PhantomData<T>,
}

impl<T: Scan> Iterator for Iter<'_, T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> { Some(self.io.scan()) }
}

pub trait Scan {
    fn scan(io: &mut IO) -> Self;
}

macro_rules! impl_parse_iint {
    ($($t:ty),*) => { $(
        impl Scan for $t {
            fn scan(s: &mut IO) -> Self {
                let mut res = 0;
                let mut inp = s.scan_raw();
                let mut neg = false;
                if inp[0] == b'-' {
                    neg = true;
                    inp = &inp[1..];
                }
                for d in inp {
                    res *= 10;
                    res += (*d - b'0') as $t;
                }
                if neg { -res } else { res }
            }
        }
    )* };
}

macro_rules! impl_parse_uint {
    ($($t:ty),*) => { $(
        impl Scan for $t {
            fn scan(s: &mut IO) -> Self {
                let mut res = 0;
                for d in s.scan_raw() {
                    res *= 10;
                    res += (*d - b'0') as $t;
                }
                res
            }
        }
    )* };
}

impl_parse_iint!(i32, i64, i128, isize);
impl_parse_uint!(u32, u64, u128, usize);

impl Scan for u8 {
    fn scan(s: &mut IO) -> Self {
        let bytes = s.scan_raw();
        debug_assert_eq!(bytes.len(), 1);
        bytes[0]
    }
}

impl Scan for &[u8] {
    fn scan(s: &mut IO) -> Self { s.scan_raw() }
}

impl<T: Scan, U: Scan> Scan for (T, U) {
    fn scan(s: &mut IO) -> Self { (T::scan(s), U::scan(s)) }
}

impl<T: Scan, U: Scan, V: Scan> Scan for (T, U, V) {
    fn scan(s: &mut IO) -> Self { (T::scan(s), U::scan(s), V::scan(s)) }
}

impl<T: Scan> Scan for [T; 2] {
    fn scan(s: &mut IO) -> Self { [s.scan(), s.scan()] }
}

impl<T: Scan> Scan for [T; 3] {
    fn scan(s: &mut IO) -> Self { [s.scan(), s.scan(), s.scan()] }
}

impl<T: Scan> Scan for [T; 4] {
    fn scan(s: &mut IO) -> Self { [s.scan(), s.scan(), s.scan(), s.scan()] }
}

pub trait Print {
    fn print(w: &mut IO, x: Self);
}

macro_rules! impl_print_int {
    ($($t:ty),*) => { $(
        impl Print for $t {
            fn print(w: &mut IO, x: Self) {
                w.buf.write_all(x.to_string().as_bytes()).unwrap();
            }
        }
    )* };
}

impl_print_int!(i32, i64, isize, u32, u64, usize);

impl Print for u8 {
    fn print(w: &mut IO, x: Self) { w.buf.write_all(&[x]).unwrap(); }
}

impl Print for &[u8] {
    fn print(w: &mut IO, x: Self) { w.buf.write_all(x).unwrap(); }
}

impl Print for &str {
    fn print(w: &mut IO, x: Self) { w.print(x.as_bytes()); }
}

impl<T: Print, U: Print> Print for (T, U) {
    fn print(w: &mut IO, (x, y): Self) { w.print(x); w.print(" "); w.print(y); }
}

impl<T: Print, U: Print, V: Print> Print for (T, U, V) {
    fn print(w: &mut IO, (x, y, z): Self) {
        w.print(x);
        w.print(" ");
        w.print(y);
        w.print(" ");
        w.print(z);
    }
}
