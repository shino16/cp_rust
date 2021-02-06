---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':warning:'
    path: src/draft/fpacc64.rs
    title: src/draft/fpacc64.rs
  - icon: ':heavy_check_mark:'
    path: src/fp.rs
    title: src/fp.rs
  - icon: ':heavy_check_mark:'
    path: src/fp/conv.rs
    title: src/fp/conv.rs
  - icon: ':warning:'
    path: src/fp/num.rs
    title: src/fp/num.rs
  - icon: ':warning:'
    path: src/graph/io.rs
    title: src/graph/io.rs
  - icon: ':warning:'
    path: src/io/graph.rs
    title: src/io/graph.rs
  - icon: ':heavy_check_mark:'
    path: src/mint.rs
    title: src/mint.rs
  - icon: ':heavy_check_mark:'
    path: src/mint/conv.rs
    title: src/mint/conv.rs
  - icon: ':warning:'
    path: src/mint/num.rs
    title: src/mint/num.rs
  - icon: ':heavy_check_mark:'
    path: src/tests.rs
    title: src/tests.rs
  - icon: ':warning:'
    path: src/u64/conv.rs
    title: src/u64/conv.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/cargo_test.rs
    title: test/src/bin/cargo_test.rs
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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/io.rs\n"
  code: "pub mod graph;\nuse std::io::{stdout, BufWriter, Read, StdoutLock, Write};\n\
    use std::marker::PhantomData;\n\npub type Bytes = &'static [u8];\n\npub struct\
    \ IO {\n\titer: std::str::SplitAsciiWhitespace<'static>,\n\tbuf: BufWriter<StdoutLock<'static>>,\n\
    }\nimpl IO {\n\tpub fn new() -> Self {\n\t\tlet mut input = String::new();\n\t\
    \tstd::io::stdin().read_to_string(&mut input).unwrap();\n\t\tlet input = Box::leak(input.into_boxed_str());\n\
    \t\tlet out = Box::leak(Box::new(stdout()));\n\t\tIO {\n\t\t\titer: input.split_ascii_whitespace(),\n\
    \t\t\tbuf: BufWriter::new(out.lock()),\n\t\t}\n\t}\n\tfn scan_str(&mut self) ->\
    \ &'static str { self.iter.next().unwrap() }\n\tfn scan_raw(&mut self) -> Bytes\
    \ { self.scan_str().as_bytes() }\n\tpub fn scan<T: Scan>(&mut self) -> T { T::scan(self)\
    \ }\n\tpub fn scan_iter<T: Scan>(&mut self, n: usize) -> std::iter::Take<Iter<'_,\
    \ T>> {\n\t\tIter { io: self, _m: PhantomData }.take(n)\n\t}\n\tpub fn scan_vec<T:\
    \ Scan>(&mut self, n: usize) -> Vec<T> {\n\t\t(0..n).map(|_| self.scan()).collect()\n\
    \t}\n\tpub fn print<T: Print>(&mut self, x: T) { T::print(self, x); }\n\tpub fn\
    \ println<T: Print>(&mut self, x: T) {\n\t\tself.print(x);\n\t\tself.print(\"\\\
    n\");\n\t}\n\tpub fn iterln<T: Print, I: IntoIterator<Item = T>>(&mut self, into_iter:\
    \ I, delim: &str) {\n\t\tlet mut iter = into_iter.into_iter();\n\t\tif let Some(v)\
    \ = iter.next() {\n\t\t\tself.print(v);\n\t\t\tfor v in iter {\n\t\t\t\tself.print(delim);\n\
    \t\t\t\tself.print(v);\n\t\t\t}\n\t\t}\n\t\tself.print(\"\\n\");\n\t}\n\tpub fn\
    \ flush(&mut self) { self.buf.flush().unwrap(); }\n}\npub struct Iter<'a, T> {\n\
    \tio: &'a mut IO,\n\t_m: PhantomData<T>,\n}\nimpl<T: Scan> Iterator for Iter<'_,\
    \ T> {\n\ttype Item = T;\n\tfn next(&mut self) -> Option<Self::Item> { Some(self.io.scan())\
    \ }\n}\npub trait Scan {\n\tfn scan(io: &mut IO) -> Self;\n}\npub trait Print\
    \ {\n\tfn print(w: &mut IO, x: Self);\n}\nmacro_rules! impl_parse_iint {\n\t($($t:ty),*)\
    \ => { $(\n\t\timpl Scan for $t {\n\t\t\tfn scan(s: &mut IO) -> Self {\n\t\t\t\
    \tlet scan = |t: &[u8]| t.iter().fold(0, |s, &b| s * 10 + (b & 0x0F) as $t);\n\
    \t\t\t\tlet s = s.scan_raw();\n\t\t\t\tif let Some((&b'-', t)) = s.split_first()\
    \ { -scan(t) } else { scan(s) }\n\t\t\t}\n\t\t}\n\t)* };\n}\nmacro_rules! impl_parse_uint\
    \ {\n\t($($t:ty),*) => { $(\n\t\timpl Scan for $t {\n\t\t\tfn scan(s: &mut IO)\
    \ -> Self {\n\t\t\t\ts.scan_raw().iter().fold(0, |s, &b| s * 10 + (b & 0x0F) as\
    \ $t)\n\t\t\t}\n\t\t}\n\t)* };\n}\nimpl_parse_iint!(i32, i64, i128, isize);\n\
    impl_parse_uint!(u32, u64, u128, usize);\nimpl Scan for u8 {\n\tfn scan(s: &mut\
    \ IO) -> Self {\n\t\tlet bytes = s.scan_raw();\n\t\tdebug_assert_eq!(bytes.len(),\
    \ 1);\n\t\tbytes[0]\n\t}\n}\nimpl Scan for Bytes {\n\tfn scan(s: &mut IO) -> Self\
    \ { s.scan_raw() }\n}\nimpl Scan for Vec<u8> {\n\tfn scan(s: &mut IO) -> Self\
    \ { s.scan_raw().to_vec() }\n}\nmacro_rules! impl_tuple {\n\t() => {};\n\t($t:ident\
    \ $($ts:ident)*) => {\n\t\timpl<$t: Scan, $($ts: Scan),*> Scan for ($t, $($ts),*)\
    \ {\n\t\t\tfn scan(s: &mut IO) -> Self { ($t::scan(s), $($ts::scan(s)),*) }\n\t\
    \t}\n\t\timpl<$t: Print, $($ts: Print),*> Print for ($t, $($ts),*) {\n\t\t\t#[allow(non_snake_case)]\n\
    \t\t\tfn print(w: &mut IO, ($t, $($ts),*): Self) {\n\t\t\t\tw.print($t);\n\t\t\
    \t\t$( w.print(\" \"); w.print($ts); )*\n\t\t\t}\n\t\t}\n\t\timpl_tuple!($($ts)*);\n\
    \t};\n}\nimpl_tuple!(A B C D E F G);\nmacro_rules! impl_scan_array {\n\t() =>\
    \ {};\n\t($n:literal $($ns:literal)*) => {\n\t\timpl<T: Scan> Scan for [T; $n]\
    \ {\n\t\t\tfn scan(s: &mut IO) -> Self {\n\t\t\t\tlet mut scan = |_| T::scan(s);\n\
    \t\t\t\t[scan($n), $(scan($ns)),*]\n\t\t\t}\n\t\t}\n\t\t// use IO::iterln to print\
    \ [T; N]\n\t\timpl_scan_array!($($ns)*);\n\t};\n}\nimpl_scan_array!(7 6 5 4 3\
    \ 2 1);\nmacro_rules! impl_print_prim {\n\t($($t:ty),*) => { $(\n\t\timpl Print\
    \ for $t {\n\t\t\tfn print(w: &mut IO, x: Self) {\n\t\t\t\tw.buf.write_all(format!(\"\
    {:.10}\", x).as_bytes()).unwrap();\n\t\t\t}\n\t\t}\n\t\timpl Print for &$t {\n\
    \t\t\tfn print(w: &mut IO, x: Self) { w.print(*x); }\n\t\t}\n\t)* };\n}\nimpl_print_prim!(i32,\
    \ i64, i128, isize, u32, u64, u128, usize, f32, f64);\nimpl Print for u8 {\n\t\
    fn print(w: &mut IO, x: Self) { w.buf.write_all(&[x]).unwrap(); }\n}\nimpl Print\
    \ for &[u8] {\n\tfn print(w: &mut IO, x: Self) { w.buf.write_all(x).unwrap();\
    \ }\n}\nimpl Print for Vec<u8> {\n\tfn print(w: &mut IO, x: Self) { w.buf.write_all(&x).unwrap();\
    \ }\n}\nimpl Print for &str {\n\tfn print(w: &mut IO, x: Self) { w.print(x.as_bytes());\
    \ }\n}\n#[derive(Debug, Clone, Copy, Default)]\npub struct Usize1(pub usize);\n\
    impl Scan for Usize1 {\n\tfn scan(io: &mut IO) -> Self {\n\t\tlet n: usize = io.scan();\n\
    \t\tSelf(n - 1)\n\t}\n}\nimpl Print for Usize1 {\n\tfn print(w: &mut IO, x: Self)\
    \ { w.print(x.0 + 1) }\n}\n\nimpl Scan for f64 {\n    fn scan(io: &mut IO) ->\
    \ Self {\n        io.scan_str().parse().unwrap()\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/io.rs
  requiredBy:
  - src/draft/fpacc64.rs
  - src/mint/num.rs
  - src/mint/conv.rs
  - src/fp/num.rs
  - src/fp/conv.rs
  - src/fp.rs
  - src/graph/io.rs
  - src/mint.rs
  - src/u64/conv.rs
  - src/io/graph.rs
  - src/tests.rs
  timestamp: '2021-02-07 05:27:00+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/lazy_segtree_test.rs
  - test/src/bin/hlpp_test.rs
  - test/src/bin/cargo_test.rs
  - test/src/bin/ntt_test.rs
  - test/src/bin/ntt_garner_test.rs
  - test/src/bin/ford_fulkerson_test.rs
  - test/src/bin/tree_dfs_io_test.rs
  - test/src/bin/edmonds_karp_test.rs
  - test/src/bin/union_find_test.rs
  - test/src/bin/segtree_test.rs
  - test/src/bin/segtree_beats_test.rs
  - test/src/bin/swag_test.rs
  - test/src/bin/ntt_mint_test.rs
  - test/src/bin/dfa_test.rs
  - test/src/bin/ntt_mint_garner_test.rs
documentation_of: src/io.rs
layout: document
redirect_from:
- /library/src/io.rs
- /library/src/io.rs.html
title: src/io.rs
---
