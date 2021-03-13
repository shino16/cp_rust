---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/graph.rs
    title: src/graph.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/graph/grid.rs\n"
  code: "pub use super::*;\n\npub struct Grid<F> {\n    pub h: usize,\n    pub w:\
    \ usize,\n    pub is_edge: F,\n    shift: u32,\n}\n\nimpl<F: Fn((usize, usize),\
    \ (usize, usize)) -> bool> Grid<F> {\n    pub fn new(h: usize, w: usize, is_edge:\
    \ F) -> Self {\n        let shift = w.next_power_of_two().trailing_zeros();\n\
    \        Self { h, w, is_edge, shift }\n    }\n    pub fn at(&self, r: usize,\
    \ c: usize) -> usize {\n        (r << self.shift) + c\n    }\n    pub fn r(&self,\
    \ v: usize) -> usize {\n        v >> self.shift\n    }\n    pub fn c(&self, v:\
    \ usize) -> usize {\n        v & ((1 << self.shift) - 1)\n    }\n    pub fn pos(&self,\
    \ v: usize) -> Option<(usize, usize)> {\n        let (r, c) = (self.r(v), self.c(v));\n\
    \        if r < self.h && c < self.w {\n            Some((r, c))\n        } else\
    \ {\n            None\n        }\n    }\n}\n\nimpl<F: Fn((usize, usize), (usize,\
    \ usize)) -> bool> Graph for Grid<F> {\n    fn len(&self) -> usize {\n       \
    \ self.h << self.shift\n    }\n    fn adj<G: FnMut(usize)>(&self, v: usize, mut\
    \ f: G) {\n        if let Some((r, c)) = self.pos(v) {\n            const DR:\
    \ [usize; 4] = [1, !0, 0, 0];\n            const DC: [usize; 4] = [0, 0, 1, !0];\n\
    \            for (&dr, &dc) in DR.iter().zip(&DC) {\n                let (r2,\
    \ c2) = (r.wrapping_add(dr), c.wrapping_add(dc));\n                if r2 < self.h\
    \ && c2 < self.w && (self.is_edge)((r, c), (r2, c2)) {\n                    f(self.at(r2,\
    \ c2));\n                }\n            }\n        }\n    }\n}\n"
  dependsOn:
  - src/graph.rs
  isVerificationFile: false
  path: src/graph/grid.rs
  requiredBy: []
  timestamp: '2021-03-14 02:25:56+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/graph/grid.rs
layout: document
redirect_from:
- /library/src/graph/grid.rs
- /library/src/graph/grid.rs.html
title: src/graph/grid.rs
---
