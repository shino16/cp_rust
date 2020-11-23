---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/cargo_test.rs
    title: test/src/bin/cargo_test.rs
  - icon: ':x:'
    path: test/src/bin/dfa_test.rs
    title: test/src/bin/dfa_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/ntt_garner_test.rs
    title: test/src/bin/ntt_garner_test.rs
  - icon: ':x:'
    path: test/src/bin/ntt_test.rs
    title: test/src/bin/ntt_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/segtree_test.rs
    title: test/src/bin/segtree_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/union_find_test.rs
    title: test/src/bin/union_find_test.rs
  _pathExtension: rs
  _verificationStatusIcon: ':question:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "use std::io::{stdout, BufWriter, Read, StdoutLock, Write};\nuse std::marker::PhantomData;\n\
    \npub struct IO {\n    iter: std::str::SplitAsciiWhitespace<'static>,\n    buf:\
    \ BufWriter<StdoutLock<'static>>,\n}\n\nimpl IO {\n    pub fn new() -> Self {\n\
    \        let mut input = String::new();\n        std::io::stdin().read_to_string(&mut\
    \ input).unwrap();\n        let input = Box::leak(input.into_boxed_str());\n \
    \       let out = Box::leak(Box::new(stdout()));\n        IO { iter: input.split_ascii_whitespace(),\
    \ buf: BufWriter::new(out.lock()) }\n    }\n    fn scan_str(&mut self) -> &'static\
    \ str { self.iter.next().unwrap() }\n    fn scan_raw(&mut self) -> &'static [u8]\
    \ { self.scan_str().as_bytes() }\n    pub fn scan<T: Scan>(&mut self) -> T { T::scan(self)\
    \ }\n    pub fn scan_iter<T: Scan>(&mut self) -> Iter<'_, T> { Iter { io: self,\
    \ _m: PhantomData } }\n    pub fn scan_n<T: Scan>(&mut self, n: usize) -> std::iter::Take<Iter<'_,\
    \ T>> {\n        self.scan_iter().take(n)\n    }\n    pub fn scan_vec<T: Scan>(&mut\
    \ self, n: usize) -> Vec<T> {\n        (0..n).map(|_| self.scan()).collect()\n\
    \    }\n\n    pub fn print<T: Print>(&mut self, x: T) { T::print(self, x); }\n\
    \    pub fn println<T: Print>(&mut self, x: T) {\n        self.print(x);\n   \
    \     self.print(\"\\n\");\n    }\n    pub fn iterln<T: Print, I: IntoIterator<Item\
    \ = T>>(&mut self, into_iter: I, delim: &str) {\n        let mut iter = into_iter.into_iter();\n\
    \        if let Some(v) = iter.next() {\n            self.print(v);\n        \
    \    for v in iter {\n                self.print(delim);\n                self.print(v);\n\
    \            }\n        }\n        self.print(\"\\n\");\n    }\n    pub fn flush(&mut\
    \ self) { self.buf.flush().unwrap(); }\n}\n\npub struct Iter<'a, T> {\n    io:\
    \ &'a mut IO,\n    _m: PhantomData<T>,\n}\n\nimpl<T: Scan> Iterator for Iter<'_,\
    \ T> {\n    type Item = T;\n    fn next(&mut self) -> Option<Self::Item> { Some(self.io.scan())\
    \ }\n}\n\npub trait Scan {\n    fn scan(io: &mut IO) -> Self;\n}\n\nmacro_rules!\
    \ impl_parse_iint {\n    ($($t:ty),*) => { $(\n        impl Scan for $t {\n  \
    \          fn scan(s: &mut IO) -> Self {\n                let mut res = 0;\n \
    \               let mut inp = s.scan_raw();\n                let mut neg = false;\n\
    \                if inp[0] == b'-' {\n                    neg = true;\n      \
    \              inp = &inp[1..];\n                }\n                for d in inp\
    \ {\n                    res *= 10;\n                    res += (*d - b'0') as\
    \ $t;\n                }\n                if neg { -res } else { res }\n     \
    \       }\n        }\n    )* };\n}\n\nmacro_rules! impl_parse_uint {\n    ($($t:ty),*)\
    \ => { $(\n        impl Scan for $t {\n            fn scan(s: &mut IO) -> Self\
    \ {\n                let mut res = 0;\n                for d in s.scan_raw() {\n\
    \                    res *= 10;\n                    res += (*d - b'0') as $t;\n\
    \                }\n                res\n            }\n        }\n    )* };\n\
    }\n\nimpl_parse_iint!(i32, i64, i128, isize);\nimpl_parse_uint!(u32, u64, u128,\
    \ usize);\n\nimpl Scan for u8 {\n    fn scan(s: &mut IO) -> Self {\n        let\
    \ bytes = s.scan_raw();\n        debug_assert_eq!(bytes.len(), 1);\n        bytes[0]\n\
    \    }\n}\n\nimpl Scan for &[u8] {\n    fn scan(s: &mut IO) -> Self { s.scan_raw()\
    \ }\n}\n\nimpl<T: Scan, U: Scan> Scan for (T, U) {\n    fn scan(s: &mut IO) ->\
    \ Self { (T::scan(s), U::scan(s)) }\n}\n\nimpl<T: Scan, U: Scan, V: Scan> Scan\
    \ for (T, U, V) {\n    fn scan(s: &mut IO) -> Self { (T::scan(s), U::scan(s),\
    \ V::scan(s)) }\n}\n\nimpl<T: Scan> Scan for [T; 2] {\n    fn scan(s: &mut IO)\
    \ -> Self { [s.scan(), s.scan()] }\n}\n\nimpl<T: Scan> Scan for [T; 3] {\n   \
    \ fn scan(s: &mut IO) -> Self { [s.scan(), s.scan(), s.scan()] }\n}\n\nimpl<T:\
    \ Scan> Scan for [T; 4] {\n    fn scan(s: &mut IO) -> Self { [s.scan(), s.scan(),\
    \ s.scan(), s.scan()] }\n}\n\npub trait Print {\n    fn print(w: &mut IO, x: Self);\n\
    }\n\nmacro_rules! impl_print_int {\n    ($($t:ty),*) => { $(\n        impl Print\
    \ for $t {\n            fn print(w: &mut IO, x: Self) {\n                w.buf.write_all(x.to_string().as_bytes()).unwrap();\n\
    \            }\n        }\n    )* };\n}\n\nimpl_print_int!(i32, i64, isize, u32,\
    \ u64, usize);\n\nimpl Print for u8 {\n    fn print(w: &mut IO, x: Self) { w.buf.write_all(&[x]).unwrap();\
    \ }\n}\n\nimpl Print for &[u8] {\n    fn print(w: &mut IO, x: Self) { w.buf.write_all(x).unwrap();\
    \ }\n}\n\nimpl Print for &str {\n    fn print(w: &mut IO, x: Self) { w.print(x.as_bytes());\
    \ }\n}\n\nimpl<T: Print, U: Print> Print for (T, U) {\n    fn print(w: &mut IO,\
    \ (x, y): Self) { w.print(x); w.print(\" \"); w.print(y); }\n}\n\nimpl<T: Print,\
    \ U: Print, V: Print> Print for (T, U, V) {\n    fn print(w: &mut IO, (x, y, z):\
    \ Self) {\n        w.print(x);\n        w.print(\" \");\n        w.print(y);\n\
    \        w.print(\" \");\n        w.print(z);\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/io.rs
  requiredBy: []
  timestamp: '2020-11-24 01:55:32+09:00'
  verificationStatus: LIBRARY_SOME_WA
  verifiedWith:
  - test/src/bin/cargo_test.rs
  - test/src/bin/segtree_test.rs
  - test/src/bin/union_find_test.rs
  - test/src/bin/dfa_test.rs
  - test/src/bin/ntt_garner_test.rs
  - test/src/bin/ntt_test.rs
documentation_of: src/io.rs
layout: document
redirect_from:
- /library/src/io.rs
- /library/src/io.rs.html
title: src/io.rs
---
