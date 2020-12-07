---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/cargo_test.rs
    title: test/src/bin/cargo_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/dfa_test.rs
    title: test/src/bin/dfa_test.rs
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
    path: test/src/bin/segtree_test.rs
    title: test/src/bin/segtree_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/union_find_test.rs
    title: test/src/bin/union_find_test.rs
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "use std::io::{stdout, BufWriter, Read, StdoutLock, Write};\nuse std::marker::PhantomData;\n\
    \npub struct IO {\n\titer: std::str::SplitAsciiWhitespace<'static>,\n\tbuf: BufWriter<StdoutLock<'static>>,\n\
    }\n\nimpl IO {\n\tpub fn new() -> Self {\n\t\tlet mut input = String::new();\n\
    \t\tstd::io::stdin().read_to_string(&mut input).unwrap();\n\t\tlet input = Box::leak(input.into_boxed_str());\n\
    \t\tlet out = Box::leak(Box::new(stdout()));\n\t\tIO { iter: input.split_ascii_whitespace(),\
    \ buf: BufWriter::new(out.lock()) }\n\t}\n\tfn scan_str(&mut self) -> &'static\
    \ str { self.iter.next().unwrap() }\n\tfn scan_raw(&mut self) -> &'static [u8]\
    \ { self.scan_str().as_bytes() }\n\tpub fn scan<T: Scan>(&mut self) -> T { T::scan(self)\
    \ }\n\tpub fn scan_iter<T: Scan>(&mut self) -> Iter<'_, T> { Iter { io: self,\
    \ _m: PhantomData } }\n\tpub fn scan_n<T: Scan>(&mut self, n: usize) -> std::iter::Take<Iter<'_,\
    \ T>> {\n\t\tself.scan_iter().take(n)\n\t}\n\tpub fn scan_vec<T: Scan>(&mut self,\
    \ n: usize) -> Vec<T> {\n\t\t(0..n).map(|_| self.scan()).collect()\n\t}\n\n\t\
    pub fn print<T: Print>(&mut self, x: T) { T::print(self, x); }\n\tpub fn println<T:\
    \ Print>(&mut self, x: T) { self.print(x); self.print(\"\\n\"); }\n\tpub fn iterln<T:\
    \ Print, I: IntoIterator<Item = T>>(&mut self, into_iter: I, delim: &str) {\n\t\
    \tlet mut iter = into_iter.into_iter();\n\t\tif let Some(v) = iter.next() {\n\t\
    \t\tself.print(v);\n\t\t\tfor v in iter { self.print(delim); self.print(v); }\n\
    \t\t}\n\t\tself.print(\"\\n\");\n\t}\n\tpub fn flush(&mut self) { self.buf.flush().unwrap();\
    \ }\n}\n\npub struct Iter<'a, T> {\n\tio: &'a mut IO,\n\t_m: PhantomData<T>,\n\
    }\n\nimpl<T: Scan> Iterator for Iter<'_, T> {\n    type Item = T;\n\tfn next(&mut\
    \ self) -> Option<Self::Item> { Some(self.io.scan()) }\n}\n\npub trait Scan {\n\
    \    fn scan(io: &mut IO) -> Self;\n}\n\npub trait Print {\n    fn print(w: &mut\
    \ IO, x: Self);\n}\n\nmacro_rules! impl_parse_iint {\n\t($($t:ty),*) => { $(\n\
    \t\timpl Scan for $t {\n\t\t\tfn scan(s: &mut IO) -> Self {\n                let\
    \ scan = |t: &[u8]| t.iter().fold(0, |s, &b| s * 10 + (b & 0x0F) as $t);\n   \
    \             let s = s.scan_raw();\n                if let Some((&b'-', t)) =\
    \ s.split_first() { -scan(t) } else { scan(s) }\n\t\t\t}\n\t\t}\n\t)* };\n}\n\n\
    macro_rules! impl_parse_uint {\n\t($($t:ty),*) => { $(\n\t\timpl Scan for $t {\n\
    \t\t\tfn scan(s: &mut IO) -> Self {\n                s.scan_raw().iter().fold(0,\
    \ |s, &b| s * 10 + (b & 0x0F) as $t)\n\t\t\t}\n\t\t}\n\t)* };\n}\n\nimpl_parse_iint!(i32,\
    \ i64, i128, isize);\nimpl_parse_uint!(u32, u64, u128, usize);\n\nimpl Scan for\
    \ u8 {\n\tfn scan(s: &mut IO) -> Self {\n        let bytes = s.scan_raw();\n\t\
    \tdebug_assert_eq!(bytes.len(), 1);\n\t\tbytes[0]\n\t}\n}\n\nimpl Scan for &[u8]\
    \ {\n\tfn scan(s: &mut IO) -> Self { s.scan_raw() }\n}\n\nmacro_rules! impl_tuple\
    \ {\n    () => {};\n    ($t:ident $($ts:ident)*) => {\n        impl<$t: Scan,\
    \ $($ts: Scan),*> Scan for ($t, $($ts),*) {\n            fn scan(s: &mut IO) ->\
    \ Self { ($t::scan(s), $($ts::scan(s)),*) }\n        }\n        impl<$t: Print,\
    \ $($ts: Print),*> Print for ($t, $($ts),*) {\n            #[allow(non_snake_case)]\n\
    \            fn print(w: &mut IO, ($t, $($ts),*): Self) {\n                w.print($t);\n\
    \                $( w.print(\" \"); w.print($ts); )*\n            }\n        }\n\
    \        impl_tuple!($($ts)*);\n    };\n}\n\nimpl_tuple!(A B C D E F G);\n\nmacro_rules!\
    \ impl_scan_array {\n    () => {};\n    ($n:literal $($ns:literal)*) => {\n  \
    \      impl<T: Scan> Scan for [T; $n] {\n            fn scan(s: &mut IO) -> Self\
    \ {\n                let mut scan = |_| T::scan(s);\n                [scan($n),\
    \ $(scan($ns)),*]\n            }\n        }\n        impl_scan_array!($($ns)*);\n\
    \    };\n}\n\nimpl_scan_array!(7 6 5 4 3 2 1);\n\nmacro_rules! impl_print_int\
    \ {\n\t($($t:ty),*) => { $(\n\t\timpl Print for $t {\n\t\t\tfn print(w: &mut IO,\
    \ x: Self) {\n\t\t\t\tw.buf.write_all(x.to_string().as_bytes()).unwrap();\n\t\t\
    \t}\n\t\t}\n\t\timpl Print for &$t {\n\t\t\tfn print(w: &mut IO, x: Self) { w.print(*x);\
    \ }\n\t\t}\n\t)* };\n}\n\nimpl_print_int!(i32, i64, i128, isize, u32, u64, u128,\
    \ usize);\n\nimpl Print for u8 {\n\tfn print(w: &mut IO, x: Self) { w.buf.write_all(&[x]).unwrap();\
    \ }\n}\n\nimpl Print for &[u8] {\n\tfn print(w: &mut IO, x: Self) { w.buf.write_all(x).unwrap();\
    \ }\n}\n\nimpl Print for &str {\n\tfn print(w: &mut IO, x: Self) { w.print(x.as_bytes());\
    \ }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/io.rs
  requiredBy: []
  timestamp: '2020-11-27 14:24:44+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/segtree_test.rs
  - test/src/bin/ntt_garner_test.rs
  - test/src/bin/dfa_test.rs
  - test/src/bin/ntt_test.rs
  - test/src/bin/ntt_mint_test.rs
  - test/src/bin/cargo_test.rs
  - test/src/bin/union_find_test.rs
  - test/src/bin/ntt_mint_garner_test.rs
documentation_of: src/io.rs
layout: document
redirect_from:
- /library/src/io.rs
- /library/src/io.rs.html
title: src/io.rs
---
