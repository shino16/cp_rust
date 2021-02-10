---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: src/alg.rs
    title: src/alg.rs
  - icon: ':question:'
    path: src/bits.rs
    title: src/bits.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':x:'
    path: test/src/bin/tree_dfs_io_test.rs
    title: test/src/bin/tree_dfs_io_test.rs
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':x:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/ds/fenwick.rs\n"
  code: "pub use crate::alg::*;\nuse crate::bits::*;\n\n#[derive(Clone)]\npub struct\
    \ FenwickTree<A: Monoid> {\n    data: Vec<A::Item>,\n    alg: A,\n}\n\nimpl<A:\
    \ Monoid> FenwickTree<A> {\n    pub fn new(mut data: Vec<A::Item>, alg: A) ->\
    \ Self {\n        let len = data.len();\n        data.insert(0, alg.unit());\n\
    \        for i in 1..=len {\n            if i + i.lsb() <= len {\n           \
    \     data[i + i.lsb()] = alg.op(data[i + i.lsb()], data[i]);\n            }\n\
    \        }\n        Self { data, alg }\n    }\n    pub fn len(&self) -> usize\
    \ {\n        self.data.len() - 1\n    }\n    pub fn clear(&mut self) {\n     \
    \   for e in &mut self.data {\n            *e = self.alg.unit();\n        }\n\
    \    }\n    pub fn add(&mut self, pos: usize, v: A::Item) {\n        let mut pos\
    \ = pos + 1;\n        while pos < self.data.len() {\n            self.data[pos]\
    \ = self.alg.op(self.data[pos], v);\n            pos += pos.lsb();\n        }\n\
    \    }\n    pub fn push(&mut self, v: A::Item) {\n        self.data.push(self.alg.unit());\n\
    \        self.add(self.data.len() - 1, v);\n    }\n    pub fn ask_prefix(&self,\
    \ mut r: usize) -> A::Item {\n        let mut res = self.alg.unit();\n       \
    \ while r != 0 {\n            res = self.alg.op(self.data[r], res);\n        \
    \    r -= r.lsb();\n        }\n        res\n    }\n    pub fn partition_point<F:\
    \ FnMut(A::Item) -> bool>(&self, mut pred: F) -> usize {\n        let mut x =\
    \ 0; // pred(&self.ask_prefix(x)) == true\n        let mut w = (self.data.len()\
    \ - 1).msb();\n        let mut l = self.alg.unit();\n        while w != 0 {\n\
    \            if x + w < self.data.len() && pred(self.alg.op(l, self.data[x + w]))\
    \ {\n                x += w;\n                l = self.alg.op(l, self.data[x +\
    \ w]);\n            }\n            w >>= 1;\n        }\n        x + 1\n    }\n\
    \    pub fn lower_bound(&self, v: A::Item) -> usize\n    where\n        A::Item:\
    \ Ord,\n    {\n        self.partition_point(|x| x < v)\n    }\n    pub fn upper_bound(&self,\
    \ v: A::Item) -> usize\n    where\n        A::Item: Ord,\n    {\n        self.partition_point(|x|\
    \ x <= v)\n    }\n}\n\nimpl<A: Group> FenwickTree<A> {\n    pub fn sub(&mut self,\
    \ pos: usize, v: A::Item) {\n        self.add(pos, self.alg.inv(v));\n    }\n\
    \    pub fn ask(&self, l: usize, r: usize) -> A::Item {\n        self.alg.op(self.alg.inv(self.ask_prefix(l)),\
    \ self.ask_prefix(r))\n    }\n}\n"
  dependsOn:
  - src/alg.rs
  - src/bits.rs
  isVerificationFile: false
  path: src/ds/fenwick.rs
  requiredBy: []
  timestamp: '2021-02-10 04:47:06+09:00'
  verificationStatus: LIBRARY_ALL_WA
  verifiedWith:
  - test/src/bin/tree_dfs_io_test.rs
documentation_of: src/ds/fenwick.rs
layout: document
redirect_from:
- /library/src/ds/fenwick.rs
- /library/src/ds/fenwick.rs.html
title: src/ds/fenwick.rs
---
