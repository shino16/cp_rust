---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "// TODO: integrate with crate::io\n\nuse std::io::{stdin, stdout, BufRead,\
    \ BufReader, BufWriter, StdinLock, StdoutLock, Write};\n\npub struct IO {\n  \
    \  input: Vec<u8>,\n    pos: usize,\n    in_buf: BufReader<StdinLock<'static>>,\n\
    \    out_buf: BufWriter<StdoutLock<'static>>,\n}\n\nimpl IO {\n    pub fn new()\
    \ -> Self {\n        let inp = Box::new(stdin());\n        let out = Box::new(stdout());\n\
    \        IO {\n            input: Vec::new(),\n            pos: 0,\n         \
    \   in_buf: BufReader::new(Box::leak(inp).lock()),\n            out_buf: BufWriter::new(Box::leak(out).lock()),\n\
    \        }\n    }\n    fn scan_bytes(&mut self) -> &[u8] {\n        loop {\n \
    \           if self.pos == self.input.len() {\n                self.input.clear();\n\
    \                self.in_buf.read_until(b'\\n', &mut self.input).unwrap();\n \
    \               self.pos = 0;\n            } else if self.input[self.pos].is_ascii_whitespace()\
    \ {\n                self.pos += 1;\n            } else {\n                break;\n\
    \            }\n        }\n        let i = self.pos;\n        while self.pos !=\
    \ self.input.len() && !self.input[self.pos].is_ascii_whitespace() {\n        \
    \    self.pos += 1;\n        }\n        &self.input[i..self.pos]\n    }\n    pub\
    \ fn scan<T: Scan>(&mut self) -> T { T::scan(self) }\n    pub fn vec<T: Scan>(&mut\
    \ self, n: usize) -> Vec<T> { (0..n).map(|_| self.scan()).collect() }\n    pub\
    \ fn graph(&mut self) -> (usize, usize, Vec<Vec<usize>>) {\n        let n = self.scan();\n\
    \        let m = self.scan();\n        let mut graph = vec![Vec::new(); n];\n\
    \        for _ in 0..m {\n            let u: usize = self.scan();\n          \
    \  let v: usize = self.scan();\n            graph[u].push(v);\n            graph[v].push(u);\n\
    \        }\n        (n, m, graph)\n    }\n    pub fn digraph(&mut self) -> (usize,\
    \ usize, Vec<Vec<usize>>) {\n        let n = self.scan();\n        let m = self.scan();\n\
    \        let mut graph = vec![Vec::new(); n];\n        for _ in 0..m {\n     \
    \       let u: usize = self.scan();\n            let v: usize = self.scan();\n\
    \            graph[u].push(v);\n        }\n        (n, m, graph)\n    }\n    pub\
    \ fn tree(&mut self) -> (usize, Vec<Vec<usize>>) {\n        let n = self.scan();\n\
    \        let mut graph = vec![Vec::new(); n];\n        for _ in 0..n - 1 {\n \
    \           let u: usize = self.scan();\n            let v: usize = self.scan();\n\
    \            graph[u].push(v);\n            graph[v].push(u);\n        }\n   \
    \     (n, graph)\n    }\n}\n\nimpl IO {\n    pub fn print<T: Print>(&mut self,\
    \ x: T) { T::print(self, x); }\n    pub fn println<T: Print>(&mut self, x: T)\
    \ { self.print(x); self.print(\"\\n\"); }\n    pub fn iterln<T: Print, I: Iterator<Item\
    \ = T>>(&mut self, mut iter: I, delim: &str) {\n        if let Some(v) = iter.next()\
    \ {\n            self.print(v);\n            for v in iter {\n               \
    \ self.print(delim);\n                self.println(v);\n            }\n      \
    \  }\n        self.print(\"\\n\");\n    }\n    pub fn flush(&mut self) { self.out_buf.flush().unwrap();\
    \ }\n}\n\npub trait Scan {\n    fn scan(io: &mut IO) -> Self;\n}\n\nmacro_rules!\
    \ impl_parse_int {\n    ($($t:tt),*) => { $(\n        impl Scan for $t {\n   \
    \         fn scan(s: &mut IO) -> Self {\n                let mut res = 0;\n  \
    \              for d in s.scan_bytes() {\n                    res *= 10;\n   \
    \                 res += (*d - b'0') as $t;\n                }\n             \
    \   res\n            }\n        }\n    )* };\n}\n\nimpl_parse_int!(i32, i64, isize,\
    \ u32, u64, usize);\n\nimpl Scan for u8 {\n    fn scan(s: &mut IO) -> Self {\n\
    \        let bytes = s.scan_bytes();\n        debug_assert_eq!(bytes.len(), 1);\n\
    \        bytes[0]\n    }\n}\n\nimpl Scan for Vec<u8> {\n    fn scan(s: &mut IO)\
    \ -> Self { s.scan_bytes().to_owned() }\n}\n\nimpl<T: Scan, U: Scan> Scan for\
    \ (T, U) {\n    fn scan(s: &mut IO) -> Self { (T::scan(s), U::scan(s)) }\n}\n\n\
    impl<T: Scan, U: Scan, V: Scan> Scan for (T, U, V) {\n    fn scan(s: &mut IO)\
    \ -> Self { (T::scan(s), U::scan(s), V::scan(s)) }\n}\n\nimpl<T: Scan> Scan for\
    \ [T; 2] {\n    fn scan(s: &mut IO) -> Self { [s.scan(), s.scan()] }\n}\n\nimpl<T:\
    \ Scan> Scan for [T; 3] {\n    fn scan(s: &mut IO) -> Self { [s.scan(), s.scan(),\
    \ s.scan()] }\n}\n\nimpl<T: Scan> Scan for [T; 4] {\n    fn scan(s: &mut IO) ->\
    \ Self { [s.scan(), s.scan(), s.scan(), s.scan()] }\n}\n\npub trait Print {\n\
    \    fn print(w: &mut IO, x: Self);\n}\n\nmacro_rules! impl_print_int {\n    ($($t:ty),*)\
    \ => { $(\n        impl Print for $t {\n            fn print(w: &mut IO, x: Self)\
    \ {\n                w.out_buf.write_all(x.to_string().as_bytes()).unwrap();\n\
    \            }\n        }\n    )* };\n}\n\nimpl_print_int!(i32, i64, isize, u32,\
    \ u64, usize);\n\nimpl Print for u8 {\n    fn print(w: &mut IO, x: Self) { w.out_buf.write_all(&[x]).unwrap();\
    \ }\n}\n\nimpl Print for &[u8] {\n    fn print(w: &mut IO, x: Self) { w.out_buf.write_all(x).unwrap();\
    \ }\n}\n\nimpl Print for &str {\n    fn print(w: &mut IO, x: Self) { w.print(x.as_bytes());\
    \ }\n}\n\nimpl<T: Print, U: Print> Print for (T, U) {\n    fn print(w: &mut IO,\
    \ (x, y): Self) { w.print(x); w.print(\" \"); w.print(y); }\n}\n\nimpl<T: Print,\
    \ U: Print, V: Print> Print for (T, U, V) {\n    fn print(w: &mut IO, (x, y, z):\
    \ Self) {\n        w.print(x);\n        w.print(\" \");\n        w.print(y);\n\
    \        w.print(\" \");\n        w.print(z);\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/io_interactive.rs
  requiredBy: []
  timestamp: '2020-11-17 18:39:28+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/io_interactive.rs
layout: document
redirect_from:
- /library/src/io_interactive.rs
- /library/src/io_interactive.rs.html
title: src/io_interactive.rs
---
