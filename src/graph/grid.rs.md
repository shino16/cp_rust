---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/graph.rs
    title: src/graph.rs
  - icon: ':heavy_check_mark:'
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
  code: "pub use super::*;\n\npub struct Grid<F: Fn(usize, usize) -> bool> {\n\tpub\
    \ h: usize,\n\tpub w: usize,\n\tpub wall: F,\n\tshift: u32,\n}\n\nimpl<F: Fn(usize,\
    \ usize) -> bool> Grid<F> {\n\tpub fn new(h: usize, w: usize, wall: F) -> Self\
    \ {\n\t\tlet shift = w.next_power_of_two().trailing_zeros();\n\t\tSelf { h, w,\
    \ wall, shift }\n\t}\n\tpub fn at(&self, r: usize, c: usize) -> usize {\n\t\t\
    (r << self.shift) + c\n\t}\n\tpub fn r(&self, v: usize) -> usize {\n\t\tv >> self.shift\n\
    \t}\n\tpub fn c(&self, v: usize) -> usize {\n\t\tv & ((1 << self.shift) - 1)\n\
    \t}\n\tpub fn pos(&self, v: usize) -> (usize, usize) {\n\t\t(self.r(v), self.c(v))\n\
    \t}\n}\n\nimpl<F: Fn(usize, usize) -> bool> Graph for Grid<F> {\n\tfn len(&self)\
    \ -> usize {\n\t\tself.h << self.shift\n\t}\n\tfn adj<G: FnMut(usize)>(&self,\
    \ v: usize, mut f: G) {\n\t\tlet (r, c) = self.pos(v);\n\t\tconst DR: [usize;\
    \ 4] = [1, !0, 0, 0];\n\t\tconst DC: [usize; 4] = [0, 0, 1, !0];\n\t\tfor (&dr,\
    \ &dc) in DR.iter().zip(&DC) {\n\t\t\tlet (r, c) = (r.wrapping_add(dr), c.wrapping_add(dc));\n\
    \t\t\tif r < self.h && c < self.w && !(self.wall)(r, c) {\n\t\t\t\tf(self.at(r,\
    \ c));\n\t\t\t}\n\t\t}\n\t}\n}\n"
  dependsOn:
  - src/graph.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/graph/grid.rs
  requiredBy: []
  timestamp: '2021-01-29 12:22:27+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/graph/grid.rs
layout: document
redirect_from:
- /library/src/graph/grid.rs
- /library/src/graph/grid.rs.html
title: src/graph/grid.rs
---
