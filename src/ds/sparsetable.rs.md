---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/alg.rs
    title: src/alg.rs
  - icon: ':heavy_check_mark:'
    path: src/bits.rs
    title: src/bits.rs
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
    RuntimeError: bundler is not specified: src/ds/sparsetable.rs\n"
  code: "pub use crate::alg::*;\nuse crate::bits::*;\n\n#[derive(Clone)]\npub struct\
    \ SparseTable<T, M> {\n    data: Vec<Vec<T>>,\n    alg: M,\n}\n\n/// M: Band (x\
    \ * x == x)\nimpl<T: Copy, M: Monoid<T>> SparseTable<T, M> {\n    pub fn new(data:\
    \ Vec<T>, alg: M) -> Self {\n        let len = data.len();\n        let height\
    \ = len.ilog2() as usize;\n        let mut data = vec![data];\n        for s in\
    \ 1..=height {\n            let w = 1 << (s - 1);\n            let mut new_row\
    \ = Vec::with_capacity(data[s - 1].len() - w);\n            for i in 0..data[s\
    \ - 1].len() - w {\n                new_row.push(alg.op(data[s - 1][i], data[s\
    \ - 1][i + w]));\n            }\n            data.push(new_row);\n        }\n\
    \        Self { data, alg }\n    }\n    pub fn ask(&self, l: usize, r: usize)\
    \ -> T {\n        if l == r {\n            self.alg.unit()\n        } else {\n\
    \            let s = (r - l).ilog2() as usize;\n            let w = 1 << s;\n\
    \            self.alg.op(self.data[s][l], self.data[s][r - w])\n        }\n  \
    \  }\n}\n"
  dependsOn:
  - src/alg.rs
  - src/bits.rs
  isVerificationFile: false
  path: src/ds/sparsetable.rs
  requiredBy: []
  timestamp: '2021-02-24 00:44:23+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/ds/sparsetable.rs
layout: document
redirect_from:
- /library/src/ds/sparsetable.rs
- /library/src/ds/sparsetable.rs.html
title: src/ds/sparsetable.rs
---
