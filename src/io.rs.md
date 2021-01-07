---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "pub mod graph;\nuse std::io::{stdout, BufWriter, Read, StdoutLock, Write};\n\
    use std::marker::PhantomData;\n\npub struct IO {\n\titer: std::str::SplitAsciiWhitespace<'static>,\n\
    \tbuf: BufWriter<StdoutLock<'static>>,\n}\n\nimpl IO {\n\tpub fn new() -> Self\
    \ {\n\t\tlet mut input = String::new();\n\t\tstd::io::stdin().read_to_string(&mut\
    \ input).unwrap();\n\t\tlet input = Box::leak(input.into_boxed_str());\n\t\tlet\
    \ out = Box::leak(Box::new(stdout()));\n\t\tIO {\n\t\t\titer: input.split_ascii_whitespace(),\n\
    \t\t\tbuf: BufWriter::new(out.lock()),\n\t\t}\n\t}\n\tfn scan_str(&mut self) ->\
    \ &'static str {\n\t\tself.iter.next().unwrap()\n\t}\n\tfn scan_raw(&mut self)\
    \ -> &'static [u8] {\n\t\tself.scan_str().as_bytes()\n\t}\n\tpub fn scan<T: Scan>(&mut\
    \ self) -> T {\n\t\tT::scan(self)\n\t}\n\tpub fn scan_iter<T: Scan>(&mut self)\
    \ -> Iter<'_, T> {\n\t\tIter { io: self, _m: PhantomData }\n\t}\n\tpub fn scan_n<T:\
    \ Scan>(&mut self, n: usize) -> std::iter::Take<Iter<'_, T>> {\n\t\tself.scan_iter().take(n)\n\
    \t}\n\tpub fn scan_vec<T: Scan>(&mut self, n: usize) -> Vec<T> {\n\t\t(0..n).map(|_|\
    \ self.scan()).collect()\n\t}\n\tpub fn scan_vv<T: Scan>(&mut self, h: usize,\
    \ w: usize) -> Vec<Vec<T>> {\n\t\t(0..h).map(|_| self.scan_vec(w)).collect()\n\
    \t}\n\tpub fn print<T: Print>(&mut self, x: T) {\n\t\tT::print(self, x);\n\t}\n\
    \tpub fn println<T: Print>(&mut self, x: T) {\n\t\tself.print(x);\n\t\tself.print(\"\
    \\n\");\n\t}\n\tpub fn iterln<T: Print, I: IntoIterator<Item = T>>(&mut self,\
    \ into_iter: I, delim: &str) {\n\t\tlet mut iter = into_iter.into_iter();\n\t\t\
    if let Some(v) = iter.next() {\n\t\t\tself.print(v);\n\t\t\tfor v in iter {\n\t\
    \t\t\tself.print(delim);\n\t\t\t\tself.print(v);\n\t\t\t}\n\t\t}\n\t\tself.print(\"\
    \\n\");\n\t}\n\tpub fn flush(&mut self) {\n\t\tself.buf.flush().unwrap();\n\t\
    }\n}\n\npub struct Iter<'a, T> {\n\tio: &'a mut IO,\n\t_m: PhantomData<T>,\n}\n\
    \nimpl<T: Scan> Iterator for Iter<'_, T> {\n\ttype Item = T;\n\tfn next(&mut self)\
    \ -> Option<Self::Item> {\n\t\tSome(self.io.scan())\n\t}\n}\n\npub trait Scan\
    \ {\n\tfn scan(io: &mut IO) -> Self;\n}\n\npub trait Print {\n\tfn print(w: &mut\
    \ IO, x: Self);\n}\n\nmacro_rules! impl_parse_iint {\n\t($($t:ty),*) => { $(\n\
    \t\timpl Scan for $t {\n\t\t\tfn scan(s: &mut IO) -> Self {\n\t\t\t\tlet scan\
    \ = |t: &[u8]| t.iter().fold(0, |s, &b| s * 10 + (b & 0x0F) as $t);\n\t\t\t\t\
    let s = s.scan_raw();\n\t\t\t\tif let Some((&b'-', t)) = s.split_first() { -scan(t)\
    \ } else { scan(s) }\n\t\t\t}\n\t\t}\n\t)* };\n}\n\nmacro_rules! impl_parse_uint\
    \ {\n\t($($t:ty),*) => { $(\n\t\timpl Scan for $t {\n\t\t\tfn scan(s: &mut IO)\
    \ -> Self {\n\t\t\t\ts.scan_raw().iter().fold(0, |s, &b| s * 10 + (b & 0x0F) as\
    \ $t)\n\t\t\t}\n\t\t}\n\t)* };\n}\n\nimpl_parse_iint!(i32, i64, i128, isize);\n\
    impl_parse_uint!(u32, u64, u128, usize);\n\n#[derive(Debug, Clone, Copy, Default)]\n\
    pub struct Usize1(pub usize);\nimpl Scan for Usize1 {\n\tfn scan(io: &mut IO)\
    \ -> Self {\n\t\tlet n: usize = io.scan();\n\t\tSelf(n - 1)\n\t}\n}\n\nimpl Scan\
    \ for u8 {\n\tfn scan(s: &mut IO) -> Self {\n\t\tlet bytes = s.scan_raw();\n\t\
    \tdebug_assert_eq!(bytes.len(), 1);\n\t\tbytes[0]\n\t}\n}\n\nimpl Scan for &[u8]\
    \ {\n\tfn scan(s: &mut IO) -> Self {\n\t\ts.scan_raw()\n\t}\n}\n\nmacro_rules!\
    \ impl_tuple {\n\t() => {};\n\t($t:ident $($ts:ident)*) => {\n\t\timpl<$t: Scan,\
    \ $($ts: Scan),*> Scan for ($t, $($ts),*) {\n\t\t\tfn scan(s: &mut IO) -> Self\
    \ { ($t::scan(s), $($ts::scan(s)),*) }\n\t\t}\n\t\timpl<$t: Print, $($ts: Print),*>\
    \ Print for ($t, $($ts),*) {\n\t\t\t#[allow(non_snake_case)]\n\t\t\tfn print(w:\
    \ &mut IO, ($t, $($ts),*): Self) {\n\t\t\t\tw.print($t);\n\t\t\t\t$( w.print(\"\
    \ \"); w.print($ts); )*\n\t\t\t}\n\t\t}\n\t\timpl_tuple!($($ts)*);\n\t};\n}\n\n\
    impl_tuple!(A B C D E F G);\n\nmacro_rules! impl_scan_array {\n\t() => {};\n\t\
    ($n:literal $($ns:literal)*) => {\n\t\timpl<T: Scan> Scan for [T; $n] {\n\t\t\t\
    fn scan(s: &mut IO) -> Self {\n\t\t\t\tlet mut scan = |_| T::scan(s);\n\t\t\t\t\
    [scan($n), $(scan($ns)),*]\n\t\t\t}\n\t\t}\n\t\timpl_scan_array!($($ns)*);\n\t\
    };\n}\n\nimpl_scan_array!(7 6 5 4 3 2 1);\n\nmacro_rules! impl_print_prim {\n\t\
    ($($t:ty),*) => { $(\n\t\timpl Print for $t {\n\t\t\tfn print(w: &mut IO, x: Self)\
    \ {\n\t\t\t\tw.buf.write_all(format!(\"{:.10}\", x).as_bytes()).unwrap();\n\t\t\
    \t}\n\t\t}\n\t\timpl Print for &$t {\n\t\t\tfn print(w: &mut IO, x: Self) { w.print(*x);\
    \ }\n\t\t}\n\t)* };\n}\n\nimpl_print_prim!(i32, i64, i128, isize, u32, u64, u128,\
    \ usize, f32, f64);\n\nimpl Print for u8 {\n\tfn print(w: &mut IO, x: Self) {\n\
    \t\tw.buf.write_all(&[x]).unwrap();\n\t}\n}\n\nimpl Print for &[u8] {\n\tfn print(w:\
    \ &mut IO, x: Self) {\n\t\tw.buf.write_all(x).unwrap();\n\t}\n}\n\nimpl Print\
    \ for &str {\n\tfn print(w: &mut IO, x: Self) {\n\t\tw.print(x.as_bytes());\n\t\
    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/io.rs
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/io.rs
layout: document
redirect_from:
- /library/src/io.rs
- /library/src/io.rs.html
title: src/io.rs
---
