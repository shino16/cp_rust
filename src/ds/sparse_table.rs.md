---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/alg.rs
    title: src/alg.rs
  - icon: ':heavy_check_mark:'
    path: src/bits.rs
    title: src/bits.rs
  - icon: ':heavy_check_mark:'
    path: src/ds.rs
    title: src/ds.rs
  - icon: ':heavy_check_mark:'
    path: src/ds/uvec.rs
    title: src/ds/uvec.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.4/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.4/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/ds/sparse_table.rs\n"
  code: "pub use crate::alg::*;\nuse crate::bits::*;\nuse crate::ds::uvec::*;\n\n\
    #[derive(Clone)]\npub struct SparseTable<T, M> {\n    data: UVec<UVec<T>>,\n \
    \   alg: M,\n}\n\n/// M: Band (x * x == x)\nimpl<T: Copy, M: Monoid<T>> SparseTable<T,\
    \ M> {\n    pub fn new(data: Vec<T>, alg: M) -> Self {\n        let height = data.len().ilog2()\
    \ as usize;\n        let mut data = uvec![UVec(data)];\n        for s in 1..=height\
    \ {\n            let w = 1 << (s - 1);\n            let mut new_row = UVec(Vec::with_capacity(data[s\
    \ - 1].len() - w));\n            for i in 0..data[s - 1].len() - w {\n       \
    \         new_row.push(alg.op(data[s - 1][i], data[s - 1][i + w]));\n        \
    \    }\n            data.push(new_row);\n        }\n        Self { data, alg }\n\
    \    }\n    pub fn ask(&self, l: usize, r: usize) -> T {\n        if l == r {\n\
    \            self.alg.unit()\n        } else {\n            let s = (r - l).ilog2()\
    \ as usize;\n            self.alg.op(self.data[s][l], self.data[s][r - (1 << s)])\n\
    \        }\n    }\n}\n"
  dependsOn:
  - src/alg.rs
  - src/bits.rs
  - src/ds.rs
  - src/ds/uvec.rs
  isVerificationFile: false
  path: src/ds/sparse_table.rs
  requiredBy: []
  timestamp: '2021-04-11 12:36:47+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/ds/sparse_table.rs
layout: document
redirect_from:
- /library/src/ds/sparse_table.rs
- /library/src/ds/sparse_table.rs.html
title: src/ds/sparse_table.rs
---
