---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':x:'
    path: test/src/bin/dfa_test.rs
    title: test/src/bin/dfa_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/segtree_test.rs
    title: test/src/bin/segtree_test.rs
  _pathExtension: rs
  _verificationStatusIcon: ':question:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "pub use crate::alg::*;\n\n#[derive(Clone)]\npub struct SegmentTree<A: Alg>\
    \ {\n    len: usize,\n    data: Vec<A::Item>,\n    alg: A,\n}\n\nimpl<A: Monoid>\
    \ SegmentTree<A> {\n    pub fn new(data: &[A::Item], alg: A) -> Self {\n     \
    \   let len = data.len();\n        let mut data = {\n            let mut data1\
    \ = Vec::with_capacity(len * 2);\n            data1.extend_from_slice(data);\n\
    \            data1.extend_from_slice(data);\n            data1\n        };\n \
    \       for i in (1..len).rev() {\n            data[i] = alg.op(&data[i << 1],\
    \ &data[i << 1 | 1]);\n        }\n        Self { len, data, alg }\n    }\n   \
    \ fn build(&mut self, mut p: usize) {\n        p >>= 1;\n        while p != 0\
    \ {\n            self.data[p] = self.alg.op(&self.data[p << 1], &self.data[p <<\
    \ 1 | 1]);\n            p >>= 1;\n        }\n    }\n    pub fn add(&mut self,\
    \ pos: usize, v: &A::Item) {\n        let p = pos + self.len;\n        self.data[p]\
    \ = self.alg.op(&self.data[p], v);\n        self.build(p);\n    }\n    pub fn\
    \ exec<F: FnOnce(&mut A::Item)>(&mut self, pos: usize, f: F) {\n        let p\
    \ = pos + self.len;\n        f(&mut self.data[p]);\n        self.build(p);\n \
    \   }\n    pub fn ask(&self, mut l: usize, mut r: usize) -> A::Item {\n      \
    \  let (mut resl, mut resr) = (self.alg.unit(), self.alg.unit());\n        l +=\
    \ self.len;\n        r += self.len;\n        while l < r {\n            if l &\
    \ 1 != 0 {\n                resl = self.alg.op(&resl, &self.data[l]);\n      \
    \          l += 1;\n            }\n            if r & 1 != 0 {\n             \
    \   resr = self.alg.op(&self.data[r - 1], &resr);\n                r -= 1;\n \
    \           }\n            l >>= 1;\n            r >>= 1;\n        }\n       \
    \ self.alg.op(&resl, &resr)\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/ds/segtree.rs
  requiredBy: []
  timestamp: '2020-11-15 11:00:40+09:00'
  verificationStatus: LIBRARY_SOME_WA
  verifiedWith:
  - test/src/bin/segtree_test.rs
  - test/src/bin/dfa_test.rs
documentation_of: src/ds/segtree.rs
layout: document
redirect_from:
- /library/src/ds/segtree.rs
- /library/src/ds/segtree.rs.html
title: src/ds/segtree.rs
---
