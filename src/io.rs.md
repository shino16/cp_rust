---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':warning:'
    path: src/draft/fpacc64.rs
    title: src/draft/fpacc64.rs
  - icon: ':warning:'
    path: src/gf/io.rs
    title: src/gf/io.rs
  - icon: ':warning:'
    path: src/graph/io.rs
    title: src/graph/io.rs
  - icon: ':warning:'
    path: src/io/graph.rs
    title: src/io/graph.rs
  - icon: ':heavy_check_mark:'
    path: src/mint/io.rs
    title: src/mint/io.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/dfa_test.rs
    title: test/src/bin/dfa_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/edmonds_karp_test.rs
    title: test/src/bin/edmonds_karp_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/ford_fulkerson_test.rs
    title: test/src/bin/ford_fulkerson_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/hlpp_test.rs
    title: test/src/bin/hlpp_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/lazy_segtree_test.rs
    title: test/src/bin/lazy_segtree_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/ntt_garner_test.rs
    title: test/src/bin/ntt_garner_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/ntt_mint_garner_test.rs
    title: test/src/bin/ntt_mint_garner_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/ntt_mint_test.rs
    title: test/src/bin/ntt_mint_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/ntt_test.rs
    title: test/src/bin/ntt_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/segtree_beats_test.rs
    title: test/src/bin/segtree_beats_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/segtree_test.rs
    title: test/src/bin/segtree_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/swag_test.rs
    title: test/src/bin/swag_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/tree_dfs_io_test.rs
    title: test/src/bin/tree_dfs_io_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/union_find_test.rs
    title: test/src/bin/union_find_test.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.5/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.5/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/io.rs\n"
  code: "pub mod graph;\nuse std::io::{stdout, BufWriter, Read, StdoutLock, Write};\n\
    use std::marker::PhantomData;\n\npub type Bytes = &'static [u8];\n\npub struct\
    \ IO {\n    iter: std::str::SplitAsciiWhitespace<'static>,\n    buf: BufWriter<StdoutLock<'static>>,\n\
    }\nimpl IO {\n    pub fn new() -> Self {\n        let mut input = String::new();\n\
    \        std::io::stdin().read_to_string(&mut input).unwrap();\n        let input\
    \ = Box::leak(input.into_boxed_str());\n        let out = Box::leak(Box::new(stdout()));\n\
    \        IO {\n            iter: input.split_ascii_whitespace(),\n           \
    \ buf: BufWriter::with_capacity(1 << 25, out.lock()),\n        }\n    }\n    fn\
    \ scan_str(&mut self) -> &'static str { self.iter.next().unwrap() }\n    fn scan_raw(&mut\
    \ self) -> Bytes { self.scan_str().as_bytes() }\n    pub fn scan<T: Scan>(&mut\
    \ self) -> T { T::scan(self) }\n    pub fn scan_iter<T: Scan>(&mut self, n: usize)\
    \ -> std::iter::Take<Iter<'_, T>> {\n        Iter { io: self, _m: PhantomData\
    \ }.take(n)\n    }\n    pub fn scan_vec<T: Scan>(&mut self, n: usize) -> Vec<T>\
    \ {\n        (0..n).map(|_| self.scan()).collect()\n    }\n    pub fn print<T:\
    \ Print>(&mut self, x: T) { T::print(self, x); }\n    pub fn println<T: Print>(&mut\
    \ self, x: T) {\n        self.print(x);\n        self.print(\"\\n\");\n    }\n\
    \    pub fn iterln<T: Print, I: IntoIterator<Item = T>>(&mut self, into_iter:\
    \ I, delim: &str) {\n        let mut iter = into_iter.into_iter();\n        if\
    \ let Some(v) = iter.next() {\n            self.print(v);\n            for v in\
    \ iter {\n                self.print(delim);\n                self.print(v);\n\
    \            }\n        }\n        self.print(\"\\n\");\n    }\n    pub fn flush(&mut\
    \ self) { self.buf.flush().unwrap(); }\n}\npub struct Iter<'a, T> {\n    io: &'a\
    \ mut IO,\n    _m: PhantomData<&'a T>,\n}\nimpl<T: Scan> Iterator for Iter<'_,\
    \ T> {\n    type Item = T;\n    fn next(&mut self) -> Option<Self::Item> { Some(self.io.scan())\
    \ }\n}\npub trait Scan {\n    fn scan(io: &mut IO) -> Self;\n}\npub trait Print\
    \ {\n    fn print(w: &mut IO, x: Self);\n}\nmacro_rules! impl_parse_iint {\n \
    \   ($($t:ty),*) => { $(\n        impl Scan for $t {\n            fn scan(s: &mut\
    \ IO) -> Self {\n                let scan = |t: &[u8]| t.iter().fold(0, |s, &b|\
    \ s * 10 + (b & 0x0F) as $t);\n                let s = s.scan_raw();\n       \
    \         if let Some((&b'-', t)) = s.split_first() { -scan(t) } else { scan(s)\
    \ }\n            }\n        }\n    )* };\n}\nmacro_rules! impl_parse_uint {\n\
    \    ($($t:ty),*) => { $(\n        impl Scan for $t {\n            fn scan(s:\
    \ &mut IO) -> Self {\n                s.scan_raw().iter().fold(0, |s, &b| s *\
    \ 10 + (b & 0x0F) as $t)\n            }\n        }\n    )* };\n}\nimpl_parse_iint!(i32,\
    \ i64, i128, isize);\nimpl_parse_uint!(u32, u64, u128, usize);\nimpl Scan for\
    \ u8 {\n    fn scan(s: &mut IO) -> Self {\n        let bytes = s.scan_raw();\n\
    \        debug_assert_eq!(bytes.len(), 1);\n        bytes[0]\n    }\n}\nimpl Scan\
    \ for Bytes {\n    fn scan(s: &mut IO) -> Self { s.scan_raw() }\n}\nimpl Scan\
    \ for Vec<u8> {\n    fn scan(s: &mut IO) -> Self { s.scan_raw().to_vec() }\n}\n\
    macro_rules! impl_tuple {\n    () => {};\n    ($t:ident $($ts:ident)*) => {\n\
    \        impl<$t: Scan, $($ts: Scan),*> Scan for ($t, $($ts),*) {\n          \
    \  fn scan(s: &mut IO) -> Self { ($t::scan(s), $($ts::scan(s)),*) }\n        }\n\
    \        impl<$t: Print, $($ts: Print),*> Print for ($t, $($ts),*) {\n       \
    \     #[allow(non_snake_case)]\n            fn print(w: &mut IO, ($t, $($ts),*):\
    \ Self) {\n                w.print($t);\n                $( w.print(\" \"); w.print($ts);\
    \ )*\n            }\n        }\n        impl_tuple!($($ts)*);\n    };\n}\nimpl_tuple!(A\
    \ B C D E F G);\nmacro_rules! impl_scan_array {\n    () => {};\n    ($n:literal\
    \ $($ns:literal)*) => {\n        impl<T: Scan> Scan for [T; $n] {\n          \
    \  fn scan(s: &mut IO) -> Self {\n                let mut scan = |_| T::scan(s);\n\
    \                [scan($n), $(scan($ns)),*]\n            }\n        }\n      \
    \  // use IO::iterln to print [T; N]\n        impl_scan_array!($($ns)*);\n   \
    \ };\n}\nimpl_scan_array!(7 6 5 4 3 2 1);\nmacro_rules! impl_print_prim {\n  \
    \  ($($t:ty),*) => { $(\n        impl Print for $t {\n            fn print(w:\
    \ &mut IO, x: Self) {\n                w.buf.write_all(format!(\"{:.10}\", x).as_bytes()).unwrap();\n\
    \            }\n        }\n        impl Print for &$t {\n            fn print(w:\
    \ &mut IO, x: Self) { w.print(*x); }\n        }\n    )* };\n}\nimpl_print_prim!(i32,\
    \ i64, i128, isize, u32, u64, u128, usize, f32, f64);\nimpl Print for u8 {\n \
    \   fn print(w: &mut IO, x: Self) { w.buf.write_all(&[x]).unwrap(); }\n}\nimpl\
    \ Print for &[u8] {\n    fn print(w: &mut IO, x: Self) { w.buf.write_all(x).unwrap();\
    \ }\n}\nimpl Print for Vec<u8> {\n    fn print(w: &mut IO, x: Self) { w.buf.write_all(&x).unwrap();\
    \ }\n}\nimpl Print for &str {\n    fn print(w: &mut IO, x: Self) { w.print(x.as_bytes());\
    \ }\n}\n#[derive(Debug, Clone, Copy, Default)]\npub struct Usize1(pub usize);\n\
    impl Scan for Usize1 {\n    fn scan(io: &mut IO) -> Self {\n        let n: usize\
    \ = io.scan();\n        Self(n - 1)\n    }\n}\nimpl Print for Usize1 {\n    fn\
    \ print(w: &mut IO, x: Self) { w.print(x.0 + 1) }\n}\n\nimpl Scan for f64 {\n\
    \    fn scan(io: &mut IO) -> Self {\n        io.scan_str().parse().unwrap()\n\
    \    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/io.rs
  requiredBy:
  - src/draft/fpacc64.rs
  - src/graph/io.rs
  - src/mint/io.rs
  - src/io/graph.rs
  - src/gf/io.rs
  timestamp: '2021-04-03 11:26:56+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/hlpp_test.rs
  - test/src/bin/ntt_garner_test.rs
  - test/src/bin/ntt_mint_garner_test.rs
  - test/src/bin/union_find_test.rs
  - test/src/bin/tree_dfs_io_test.rs
  - test/src/bin/ford_fulkerson_test.rs
  - test/src/bin/ntt_test.rs
  - test/src/bin/edmonds_karp_test.rs
  - test/src/bin/ntt_mint_test.rs
  - test/src/bin/dfa_test.rs
  - test/src/bin/segtree_beats_test.rs
  - test/src/bin/lazy_segtree_test.rs
  - test/src/bin/swag_test.rs
  - test/src/bin/segtree_test.rs
documentation_of: src/io.rs
layout: document
redirect_from:
- /library/src/io.rs
- /library/src/io.rs.html
title: src/io.rs
---
