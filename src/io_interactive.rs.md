---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/io_interactive.rs\n"
  code: "use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, StdinLock, StdoutLock,\
    \ Write};\nuse std::marker::PhantomData;\n\npub struct IO {\n\tinput: Vec<u8>,\n\
    \tpos: usize,\n\tin_buf: BufReader<StdinLock<'static>>,\n\tout_buf: BufWriter<StdoutLock<'static>>,\n\
    }\n\nimpl IO {\n\tpub fn new() -> Self {\n\t\tlet inp = Box::leak(Box::new(stdin()));\n\
    \t\tlet out = Box::leak(Box::new(stdout()));\n\t\tIO {\n\t\t\tinput: Vec::new(),\n\
    \t\t\tpos: 0,\n\t\t\tin_buf: BufReader::new(inp.lock()),\n\t\t\tout_buf: BufWriter::new(out.lock()),\n\
    \t\t}\n\t}\n\tfn scan_raw(&mut self) -> &[u8] {\n\t\tloop {\n\t\t\tif self.pos\
    \ == self.input.len() {\n\t\t\t\tself.input.clear();\n\t\t\t\tself.in_buf.read_until(b'\\\
    n', &mut self.input).unwrap();\n\t\t\t\tself.pos = 0;\n\t\t\t} else if self.input[self.pos].is_ascii_whitespace()\
    \ {\n\t\t\t\tself.pos += 1;\n\t\t\t} else {\n\t\t\t\tbreak;\n\t\t\t}\n\t\t}\n\t\
    \tlet i = self.pos;\n\t\twhile self.pos != self.input.len() && !self.input[self.pos].is_ascii_whitespace()\
    \ {\n\t\t\tself.pos += 1;\n\t\t}\n\t\t&self.input[i..self.pos]\n\t}\n\tpub fn\
    \ scan<T: Scan>(&mut self) -> T {\n\t\tT::scan(self)\n\t}\n\tpub fn scan_iter<T:\
    \ Scan>(&mut self) -> Iter<'_, T> {\n\t\tIter { io: self, _m: PhantomData }\n\t\
    }\n\tpub fn scan_n<T: Scan>(&mut self, n: usize) -> std::iter::Take<Iter<'_, T>>\
    \ {\n\t\tself.scan_iter().take(n)\n\t}\n\tpub fn scan_vec<T: Scan>(&mut self,\
    \ n: usize) -> Vec<T> {\n\t\t(0..n).map(|_| self.scan()).collect()\n\t}\n\n\t\
    pub fn print<T: Print>(&mut self, x: T) {\n\t\tT::print(self, x);\n\t}\n\tpub\
    \ fn println<T: Print>(&mut self, x: T) {\n\t\tself.print(x);\n\t\tself.print(\"\
    \\n\");\n\t}\n\tpub fn iterln<T: Print, I: Iterator<Item = T>>(&mut self, mut\
    \ iter: I, delim: &str) {\n\t\tif let Some(v) = iter.next() {\n\t\t\tself.print(v);\n\
    \t\t\tfor v in iter {\n\t\t\t\tself.print(delim);\n\t\t\t\tself.println(v);\n\t\
    \t\t}\n\t\t}\n\t\tself.print(\"\\n\");\n\t}\n\tpub fn flush(&mut self) {\n\t\t\
    self.out_buf.flush().unwrap();\n\t}\n}\n\npub struct Iter<'a, T> {\n\tio: &'a\
    \ mut IO,\n\t_m: PhantomData<T>,\n}\n\nimpl<T: Scan> Iterator for Iter<'_, T>\
    \ {\n\ttype Item = T;\n\tfn next(&mut self) -> Option<Self::Item> {\n\t\tSome(self.io.scan())\n\
    \t}\n}\n\npub trait Scan {\n\tfn scan(io: &mut IO) -> Self;\n}\n\npub trait Print\
    \ {\n\tfn print(w: &mut IO, x: Self);\n}\n\nmacro_rules! impl_parse_iint {\n\t\
    ($($t:ty),*) => { $(\n\t\timpl Scan for $t {\n\t\t\tfn scan(s: &mut IO) -> Self\
    \ {\n\t\t\t\tlet scan = |t: &[u8]| t.iter().fold(0, |s, &b| s * 10 + (b & 0x0F)\
    \ as $t);\n\t\t\t\tlet s = s.scan_raw();\n\t\t\t\tif let Some((&b'-', t)) = s.split_first()\
    \ { -scan(t) } else { scan(s) }\n\t\t\t}\n\t\t}\n\t)* };\n}\n\nmacro_rules! impl_parse_uint\
    \ {\n\t($($t:ty),*) => { $(\n\t\timpl Scan for $t {\n\t\t\tfn scan(s: &mut IO)\
    \ -> Self {\n\t\t\t\ts.scan_raw().iter().fold(0, |s, &b| s * 10 + (b & 0x0F) as\
    \ $t)\n\t\t\t}\n\t\t}\n\t)* };\n}\n\nimpl_parse_iint!(i32, i64, i128, isize);\n\
    impl_parse_uint!(u32, u64, u128, usize);\n\nimpl Scan for u8 {\n\tfn scan(s: &mut\
    \ IO) -> Self {\n\t\tlet bytes = s.scan_raw();\n\t\tdebug_assert_eq!(bytes.len(),\
    \ 1);\n\t\tbytes[0]\n\t}\n}\n\nimpl Scan for Vec<u8> {\n\tfn scan(s: &mut IO)\
    \ -> Self {\n\t\ts.scan_raw().to_owned()\n\t}\n}\n\nmacro_rules! impl_tuple {\n\
    \t() => {};\n\t($t:ident $($ts:ident)*) => {\n\t\timpl<$t: Scan, $($ts: Scan),*>\
    \ Scan for ($t, $($ts),*) {\n\t\t\tfn scan(s: &mut IO) -> Self { ($t::scan(s),\
    \ $($ts::scan(s)),*) }\n\t\t}\n\t\timpl<$t: Print, $($ts: Print),*> Print for\
    \ ($t, $($ts),*) {\n\t\t\t#[allow(non_snake_case)]\n\t\t\tfn print(w: &mut IO,\
    \ ($t, $($ts),*): Self) {\n\t\t\t\tw.print($t);\n\t\t\t\t$( w.print(\" \"); w.print($ts);\
    \ )*\n\t\t\t}\n\t\t}\n\t\timpl_tuple!($($ts)*);\n\t};\n}\n\nimpl_tuple!(A B C\
    \ D E F G);\n\nmacro_rules! impl_scan_array {\n\t() => {};\n\t($n:literal $($ns:literal)*)\
    \ => {\n\t\timpl<T: Scan> Scan for [T; $n] {\n\t\t\tfn scan(s: &mut IO) -> Self\
    \ {\n\t\t\t\tlet mut scan = |_| T::scan(s);\n\t\t\t\t[scan($n), $(scan($ns)),*]\n\
    \t\t\t}\n\t\t}\n\t\timpl_scan_array!($($ns)*);\n\t};\n}\n\nimpl_scan_array!(7\
    \ 6 5 4 3 2 1);\n\nmacro_rules! impl_print_int {\n\t($($t:ty),*) => { $(\n\t\t\
    impl Print for $t {\n\t\t\tfn print(w: &mut IO, x: Self) {\n\t\t\t\tw.out_buf.write_all(x.to_string().as_bytes()).unwrap();\n\
    \t\t\t}\n\t\t}\n\t\timpl Print for &$t {\n\t\t\tfn print(w: &mut IO, x: Self)\
    \ {\n\t\t\t\tw.out_buf.write_all(x.to_string().as_bytes()).unwrap();\n\t\t\t}\n\
    \t\t}\n\t)* };\n}\n\nimpl_print_int!(i32, i64, i128, isize, u32, u64, u128, usize);\n\
    \nimpl Print for u8 {\n\tfn print(w: &mut IO, x: Self) {\n\t\tw.out_buf.write_all(&[x]).unwrap();\n\
    \t}\n}\n\nimpl Print for &[u8] {\n\tfn print(w: &mut IO, x: Self) {\n\t\tw.out_buf.write_all(x).unwrap();\n\
    \t}\n}\n\nimpl Print for &str {\n\tfn print(w: &mut IO, x: Self) {\n\t\tw.print(x.as_bytes());\n\
    \t}\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/io_interactive.rs
  requiredBy: []
  timestamp: '2020-12-10 17:35:58+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/io_interactive.rs
layout: document
redirect_from:
- /library/src/io_interactive.rs
- /library/src/io_interactive.rs.html
title: src/io_interactive.rs
---