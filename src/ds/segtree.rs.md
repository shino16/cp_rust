---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/alg.rs
    title: src/alg.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/segtree_test.rs
    title: test/src/bin/segtree_test.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/ds/segtree.rs\n"
  code: "pub mod beats;\npub mod lazy;\npub use crate::alg::*;\nuse std::ops::Index;\n\
    use std::slice::SliceIndex;\n\n#[derive(Clone)]\npub struct SegmentTree<T, M>\
    \ {\n    len: usize,\n    data: Vec<T>,\n    alg: M,\n}\n\nimpl<T: Copy, M: Monoid<T>>\
    \ SegmentTree<T, M> {\n    pub fn new(len: usize, alg: M) -> Self { Self { len,\
    \ data: vec![alg.unit(); len * 2], alg } }\n    pub fn from_slice(slice: &[T],\
    \ alg: M) -> Self {\n        let len = slice.len();\n        let mut data = slice.to_vec();\n\
    \        data.extend_from_slice(slice);\n        for i in (1..len).rev() {\n \
    \           data[i] = alg.op(data[i << 1], data[i << 1 | 1]);\n        }\n   \
    \     Self { len, data, alg }\n    }\n    pub fn len(&self) -> usize { self.len\
    \ }\n    fn build(&mut self, mut p: usize) {\n        p >>= 1;\n        while\
    \ p != 0 {\n            self.data[p] = self.alg.op(self.data[p << 1], self.data[p\
    \ << 1 | 1]);\n            p >>= 1;\n        }\n    }\n    pub fn add(&mut self,\
    \ pos: usize, v: T) {\n        let p = pos + self.len();\n        self.data[p]\
    \ = self.alg.op(self.data[p], v);\n        self.build(p);\n    }\n    pub fn with<F:\
    \ FnOnce(&mut T) -> R, R>(&mut self, pos: usize, f: F) -> R {\n        let p =\
    \ pos + self.len();\n        let r = f(&mut self.data[p]);\n        self.build(p);\n\
    \        r\n    }\n    pub fn ask(&self, mut l: usize, mut r: usize) -> T {\n\
    \        let (mut resl, mut resr) = (self.alg.unit(), self.alg.unit());\n    \
    \    l += self.len();\n        r += self.len();\n        while l < r {\n     \
    \       if l & 1 != 0 {\n                resl = self.alg.op(resl, self.data[l]);\n\
    \                l += 1;\n            }\n            if r & 1 != 0 {\n       \
    \         resr = self.alg.op(self.data[r - 1], resr);\n                r -= 1;\n\
    \            }\n            l >>= 1;\n            r >>= 1;\n        }\n      \
    \  self.alg.op(resl, resr)\n    }\n}\n\nimpl<T: Copy, M: Monoid<T>, I: SliceIndex<[T]>>\
    \ Index<I> for SegmentTree<T, M> {\n    type Output = I::Output;\n    fn index(&self,\
    \ idx: I) -> &Self::Output { &self.data[self.len()..][idx] }\n}\n"
  dependsOn:
  - src/alg.rs
  isVerificationFile: false
  path: src/ds/segtree.rs
  requiredBy: []
  timestamp: '2021-02-22 02:21:06+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/segtree_test.rs
documentation_of: src/ds/segtree.rs
layout: document
redirect_from:
- /library/src/ds/segtree.rs
- /library/src/ds/segtree.rs.html
title: src/ds/segtree.rs
---
