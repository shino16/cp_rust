---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/alg.rs
    title: src/alg.rs
  - icon: ':question:'
    path: src/bit.rs
    title: src/bit.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/ds/disjointst.rs\n"
  code: "pub use crate::alg::*;\nuse crate::bit::*;\n\n#[derive(Clone)]\npub struct\
    \ DisjointSparseTable<A: Alg> {\n\tdata: Vec<Vec<A::Item>>,\n\talg: A,\n}\n\n\
    impl<A: Monoid> DisjointSparseTable<A> {\n\tpub fn new(data: Vec<A::Item>, alg:\
    \ A) -> Self {\n\t\tlet len = data.len();\n\t\tlet height = len.ilog2() as usize;\n\
    \t\tlet mut data = vec![data; height + 1];\n\t\tfor s in 1..=height {\n\t\t\t\
    for z in (0..len).step_by(1 << (s + 1)) {\n\t\t\t\tlet m = z + (1 << s);\n\t\t\
    \t\tif m >= len {\n\t\t\t\t\tbreak;\n\t\t\t\t}\n\t\t\t\tdata[s][m - 1] = data[0][m\
    \ - 1];\n\t\t\t\tdata[s][m] = data[0][m];\n\t\t\t\tfor i in (z..m - 1).rev() {\n\
    \t\t\t\t\tdata[s][i] = alg.op(data[0][i], data[s][i + 1]);\n\t\t\t\t}\n\t\t\t\t\
    for i in m + 1..(m + (1 << s)).min(len) {\n\t\t\t\t\tdata[s][i] = alg.op(data[s][i\
    \ - 1], data[0][i]);\n\t\t\t\t}\n\t\t\t}\n\t\t}\n\t\tSelf { data, alg }\n\t}\n\
    \tpub fn ask(&self, l: usize, r: usize) -> A::Item {\n\t\tif l == r {\n\t\t\t\
    self.alg.unit()\n\t\t} else if l + 1 == r {\n\t\t\tself.data[0][l]\n\t\t} else\
    \ {\n\t\t\tlet s = (l ^ r).ilog2() as usize;\n\t\t\tself.alg.op(self.data[s][l],\
    \ self.data[s][r])\n\t\t}\n\t}\n}\n"
  dependsOn:
  - src/alg.rs
  - src/bit.rs
  isVerificationFile: false
  path: src/ds/disjointst.rs
  requiredBy: []
  timestamp: '2021-01-03 22:19:36+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/ds/disjointst.rs
layout: document
redirect_from:
- /library/src/ds/disjointst.rs
- /library/src/ds/disjointst.rs.html
title: src/ds/disjointst.rs
---
