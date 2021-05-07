---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.5/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.5/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/io_interactive.rs\n"
  code: "use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, StdinLock, StdoutLock,\
    \ Write};\nuse std::marker::PhantomData;\n\npub struct IO {\n    input: Vec<u8>,\n\
    \    pos: usize,\n    in_buf: BufReader<StdinLock<'static>>,\n    out_buf: BufWriter<StdoutLock<'static>>,\n\
    }\n\nimpl IO {\n    pub fn new() -> Self {\n        let inp = Box::leak(Box::new(stdin()));\n\
    \        let out = Box::leak(Box::new(stdout()));\n        IO {\n            input:\
    \ Vec::new(),\n            pos: 0,\n            in_buf: BufReader::new(inp.lock()),\n\
    \            out_buf: BufWriter::new(out.lock()),\n        }\n    }\n    fn scan_raw(&mut\
    \ self) -> &[u8] {\n        loop {\n            if self.pos == self.input.len()\
    \ {\n                self.input.clear();\n                self.in_buf.read_until(b'\\\
    n', &mut self.input).unwrap();\n                self.pos = 0;\n            } else\
    \ if self.input[self.pos].is_ascii_whitespace() {\n                self.pos +=\
    \ 1;\n            } else {\n                break;\n            }\n        }\n\
    \        let i = self.pos;\n        while self.pos != self.input.len() && !self.input[self.pos].is_ascii_whitespace()\
    \ {\n            self.pos += 1;\n        }\n        &self.input[i..self.pos]\n\
    \    }\n    pub fn scan<T: Scan>(&mut self) -> T {\n        T::scan(self)\n  \
    \  }\n    pub fn scan_iter<T: Scan>(&mut self, n: usize) -> std::iter::Take<Iter<'_,\
    \ T>> {\n        Iter { io: self, _m: PhantomData }.take(n)\n    }\n    pub fn\
    \ scan_vec<T: Scan>(&mut self, n: usize) -> Vec<T> {\n        (0..n).map(|_| self.scan()).collect()\n\
    \    }\n    pub fn print<T: Print>(&mut self, x: T) {\n        T::print(self,\
    \ x);\n    }\n    pub fn println<T: Print>(&mut self, x: T) {\n        self.print(x);\n\
    \        self.print(\"\\n\");\n    }\n    pub fn iterln<T: Print, I: Iterator<Item\
    \ = T>>(&mut self, mut iter: I, delim: &str) {\n        if let Some(v) = iter.next()\
    \ {\n            self.print(v);\n            for v in iter {\n               \
    \ self.print(delim);\n                self.print(v);\n            }\n        }\n\
    \        self.print(\"\\n\");\n    }\n    pub fn flush(&mut self) {\n        self.out_buf.flush().unwrap();\n\
    \    }\n}\n\npub struct Iter<'a, T> {\n    io: &'a mut IO,\n    _m: PhantomData<&'a\
    \ T>,\n}\n\nimpl<T: Scan> Iterator for Iter<'_, T> {\n    type Item = T;\n   \
    \ fn next(&mut self) -> Option<Self::Item> {\n        Some(self.io.scan())\n \
    \   }\n}\n\npub trait Scan {\n    fn scan(io: &mut IO) -> Self;\n}\n\npub trait\
    \ Print {\n    fn print(w: &mut IO, x: Self);\n}\n\nmacro_rules! impl_parse_iint\
    \ {\n    ($($t:ty),*) => { $(\n        impl Scan for $t {\n            fn scan(s:\
    \ &mut IO) -> Self {\n                let scan = |t: &[u8]| t.iter().fold(0, |s,\
    \ &b| s * 10 + (b & 0x0F) as $t);\n                let s = s.scan_raw();\n   \
    \             if let Some((&b'-', t)) = s.split_first() { -scan(t) } else { scan(s)\
    \ }\n            }\n        }\n    )* };\n}\n\nmacro_rules! impl_parse_uint {\n\
    \    ($($t:ty),*) => { $(\n        impl Scan for $t {\n            fn scan(s:\
    \ &mut IO) -> Self {\n                s.scan_raw().iter().fold(0, |s, &b| s *\
    \ 10 + (b & 0x0F) as $t)\n            }\n        }\n    )* };\n}\n\nimpl_parse_iint!(i32,\
    \ i64, i128, isize);\nimpl_parse_uint!(u32, u64, u128, usize);\n\nimpl Scan for\
    \ u8 {\n    fn scan(s: &mut IO) -> Self {\n        let bytes = s.scan_raw();\n\
    \        debug_assert_eq!(bytes.len(), 1);\n        bytes[0]\n    }\n}\n\nimpl\
    \ Scan for Vec<u8> {\n    fn scan(s: &mut IO) -> Self {\n        s.scan_raw().to_owned()\n\
    \    }\n}\n\nmacro_rules! impl_tuple {\n    () => {};\n    ($t:ident $($ts:ident)*)\
    \ => {\n        impl<$t: Scan, $($ts: Scan),*> Scan for ($t, $($ts),*) {\n   \
    \         fn scan(s: &mut IO) -> Self { ($t::scan(s), $($ts::scan(s)),*) }\n \
    \       }\n        impl<$t: Print, $($ts: Print),*> Print for ($t, $($ts),*) {\n\
    \            #[allow(non_snake_case)]\n            fn print(w: &mut IO, ($t, $($ts),*):\
    \ Self) {\n                w.print($t);\n                $( w.print(\" \"); w.print($ts);\
    \ )*\n            }\n        }\n        impl_tuple!($($ts)*);\n    };\n}\n\nimpl_tuple!(A\
    \ B C D E F G);\n\nmacro_rules! impl_scan_array {\n    () => {};\n    ($n:literal\
    \ $($ns:literal)*) => {\n        impl<T: Scan> Scan for [T; $n] {\n          \
    \  fn scan(s: &mut IO) -> Self {\n                let mut scan = |_| T::scan(s);\n\
    \                [scan($n), $(scan($ns)),*]\n            }\n        }\n      \
    \  impl_scan_array!($($ns)*);\n    };\n}\n\nimpl_scan_array!(7 6 5 4 3 2 1);\n\
    \nmacro_rules! impl_print_int {\n    ($($t:ty),*) => { $(\n        impl Print\
    \ for $t {\n            fn print(w: &mut IO, x: Self) {\n                w.out_buf.write_all(x.to_string().as_bytes()).unwrap();\n\
    \            }\n        }\n        impl Print for &$t {\n            fn print(w:\
    \ &mut IO, x: Self) {\n                w.out_buf.write_all(x.to_string().as_bytes()).unwrap();\n\
    \            }\n        }\n    )* };\n}\n\nimpl_print_int!(i32, i64, i128, isize,\
    \ u32, u64, u128, usize);\n\nimpl Print for u8 {\n    fn print(w: &mut IO, x:\
    \ Self) {\n        w.out_buf.write_all(&[x]).unwrap();\n    }\n}\n\nimpl Print\
    \ for &[u8] {\n    fn print(w: &mut IO, x: Self) {\n        w.out_buf.write_all(x).unwrap();\n\
    \    }\n}\n\nimpl Print for &str {\n    fn print(w: &mut IO, x: Self) {\n    \
    \    w.print(x.as_bytes());\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/io_interactive.rs
  requiredBy: []
  timestamp: '2021-04-03 11:26:56+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/io_interactive.rs
layout: document
redirect_from:
- /library/src/io_interactive.rs
- /library/src/io_interactive.rs.html
title: src/io_interactive.rs
---
