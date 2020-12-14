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
  code: "pub use crate::graph::*;\n\npub struct Grid {\n\tpub h: usize,\n\tpub w:\
    \ usize,\n}\n\npub struct GridAdj((usize, usize), (usize, usize), usize);\nimpl\
    \ Iterator for GridAdj {\n    type Item = (usize, usize);\n    fn next(&mut self)\
    \ -> Option<Self::Item> {\n\t\tconst DR: [usize; 4] = [!0, 1, 0, 0];\n\t\tconst\
    \ DC: [usize; 4] = [0, 0, !0, 1];\n\t\tlet (r, c) = self.0;\n\t\tlet (h, w) =\
    \ self.1;\n\t\tlet mut i = self.2;\n\t\tloop {\n\t\t\ti = i.wrapping_add(1);\n\
    \t\t\tif i >= 4 {\n\t\t\t\treturn None;\n\t\t\t}\n\t\t\tlet (r, c) = (r.wrapping_add(DR[i]),\
    \ c.wrapping_add(DC[i]));\n\t\t\tif r < h && c < w {\n\t\t\t\treturn Some((r,\
    \ c));\n\t\t\t}\n\t\t}\n    }\n}\n\nimpl<'g> Graph<'g> for Grid {\n    type V\
    \ = (usize, usize);\n    type E = ();\n    type VIter = GridAdj;\n    type EIter\
    \ = std::iter::Take<std::iter::Repeat<()>>;\n    fn add_edge(&mut self, _: Self::V,\
    \ _: Self::V, _: Self::E) {\n\t\tpanic!()\n\t}\n    fn adj_v<'a: 'g>(&'a self,\
    \ v: Self::V) -> Self::VIter {\n\t\tGridAdj(v, self.dim(), !0)\n    }\n    fn\
    \ adj_e<'a: 'g>(&'a self, _: Self::V) -> Self::EIter {\n\t\tstd::iter::repeat(()).take(4)\n\
    \    }\n    fn dim(&self) -> Self::V {\n\t\t(self.h, self.w)\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/draft/graph/grid.rs
  requiredBy: []
  timestamp: '2020-12-15 00:46:43+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/draft/graph/grid.rs
layout: document
redirect_from:
- /library/src/draft/graph/grid.rs
- /library/src/draft/graph/grid.rs.html
title: src/draft/graph/grid.rs
---
