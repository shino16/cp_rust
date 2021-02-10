---
data:
  _extendedDependsOn:
  - icon: ':x:'
    path: src/graph.rs
    title: src/graph.rs
  - icon: ':question:'
    path: src/zo.rs
    title: src/zo.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/graph/grid.rs\n"
  code: "pub use super::*;\n\npub struct Grid<F: Fn(usize, usize) -> bool> {\n   \
    \ pub h: usize,\n    pub w: usize,\n    pub wall: F,\n    shift: u32,\n}\n\nimpl<F:\
    \ Fn(usize, usize) -> bool> Grid<F> {\n    pub fn new(h: usize, w: usize, wall:\
    \ F) -> Self {\n        let shift = w.next_power_of_two().trailing_zeros();\n\
    \        Self { h, w, wall, shift }\n    }\n    pub fn at(&self, r: usize, c:\
    \ usize) -> usize {\n        (r << self.shift) + c\n    }\n    pub fn r(&self,\
    \ v: usize) -> usize {\n        v >> self.shift\n    }\n    pub fn c(&self, v:\
    \ usize) -> usize {\n        v & ((1 << self.shift) - 1)\n    }\n    pub fn pos(&self,\
    \ v: usize) -> (usize, usize) {\n        (self.r(v), self.c(v))\n    }\n}\n\n\
    impl<F: Fn(usize, usize) -> bool> Graph for Grid<F> {\n    fn len(&self) -> usize\
    \ {\n        self.h << self.shift\n    }\n    fn adj<G: FnMut(usize)>(&self, v:\
    \ usize, mut f: G) {\n        let (r, c) = self.pos(v);\n        const DR: [usize;\
    \ 4] = [1, !0, 0, 0];\n        const DC: [usize; 4] = [0, 0, 1, !0];\n       \
    \ for (&dr, &dc) in DR.iter().zip(&DC) {\n            let (r, c) = (r.wrapping_add(dr),\
    \ c.wrapping_add(dc));\n            if r < self.h && c < self.w && !(self.wall)(r,\
    \ c) {\n                f(self.at(r, c));\n            }\n        }\n    }\n}\n"
  dependsOn:
  - src/graph.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/graph/grid.rs
  requiredBy: []
  timestamp: '2021-02-08 00:55:24+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/graph/grid.rs
layout: document
redirect_from:
- /library/src/graph/grid.rs
- /library/src/graph/grid.rs.html
title: src/graph/grid.rs
---
