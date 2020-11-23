use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, StdinLock, StdoutLock, Write};
use std::marker::PhantomData;

pub struct IO {
    input: Vec<u8>,
    pos: usize,
    in_buf: BufReader<StdinLock<'static>>,
    out_buf: BufWriter<StdoutLock<'static>>,
}

impl IO {
    pub fn new() -> Self {
        let inp = Box::leak(Box::new(stdin()));
        let out = Box::leak(Box::new(stdout()));
        IO {
            input: Vec::new(),
            pos: 0,
            in_buf: BufReader::new(inp.lock()),
            out_buf: BufWriter::new(out.lock()),
        }
    }
    fn scan_bytes(&mut self) -> &[u8] {
        loop {
            if self.pos == self.input.len() {
                self.input.clear();
                self.in_buf.read_until(b'\n', &mut self.input).unwrap();
                self.pos = 0;
            } else if self.input[self.pos].is_ascii_whitespace() {
                self.pos += 1;
            } else {
                break;
            }
        }
        let i = self.pos;
        while self.pos != self.input.len() && !self.input[self.pos].is_ascii_whitespace() {
            self.pos += 1;
        }
        &self.input[i..self.pos]
    }
    pub fn scan<T: Scan>(&mut self) -> T { T::scan(self) }
    pub fn scan_iter<T: Scan>(&mut self) -> Iter<'_, T> { Iter { io: self, _m: PhantomData } }
    pub fn scan_n<T: Scan>(&mut self, n: usize) -> std::iter::Take<Iter<'_, T>> {
        self.scan_iter().take(n)
    }
    pub fn scan_vec<T: Scan>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.scan()).collect()
    }

    pub fn print<T: Print>(&mut self, x: T) { T::print(self, x); }
    pub fn println<T: Print>(&mut self, x: T) { self.print(x); self.print("\n"); }
    pub fn iterln<T: Print, I: Iterator<Item = T>>(&mut self, mut iter: I, delim: &str) {
        if let Some(v) = iter.next() {
            self.print(v);
            for v in iter {
                self.print(delim);
                self.println(v);
            }
        }
        self.print("\n");
    }
    pub fn flush(&mut self) { self.out_buf.flush().unwrap(); }
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

macro_rules! impl_parse_int {
    ($($t:tt),*) => { $(
        impl Scan for $t {
            fn scan(s: &mut IO) -> Self {
                let mut res = 0;
                for d in s.scan_bytes() {
                    res *= 10;
                    res += (*d - b'0') as $t;
                }
                res
            }
        }
    )* };
}

impl_parse_int!(i32, i64, isize, u32, u64, usize);

impl Scan for u8 {
    fn scan(s: &mut IO) -> Self {
        let bytes = s.scan_bytes();
        debug_assert_eq!(bytes.len(), 1);
        bytes[0]
    }
}

impl Scan for Vec<u8> {
    fn scan(s: &mut IO) -> Self { s.scan_bytes().to_owned() }
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
                w.out_buf.write_all(x.to_string().as_bytes()).unwrap();
            }
        }
    )* };
}

impl_print_int!(i32, i64, isize, u32, u64, usize);

impl Print for u8 {
    fn print(w: &mut IO, x: Self) { w.out_buf.write_all(&[x]).unwrap(); }
}

impl Print for &[u8] {
    fn print(w: &mut IO, x: Self) { w.out_buf.write_all(x).unwrap(); }
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
