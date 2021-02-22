---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/alg.rs
    title: src/alg.rs
  - icon: ':heavy_check_mark:'
    path: src/alg/arith.rs
    title: src/alg/arith.rs
  - icon: ':heavy_check_mark:'
    path: src/num.rs
    title: src/num.rs
  - icon: ':heavy_check_mark:'
    path: src/zo.rs
    title: src/zo.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/tree_dfs_io_test.rs
    title: test/src/bin/tree_dfs_io_test.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/ds/fenwick.rs\n"
  code: "pub use crate::alg::arith::*;\n\n#[derive(Clone)]\npub struct FenwickTree<T,\
    \ M> {\n    data: Vec<T>,\n    alg: M,\n}\n\nimpl<T: Copy, M: Monoid<T>> FenwickTree<T,\
    \ M> {\n    pub fn new(mut data: Vec<T>, alg: M) -> Self {\n        let len =\
    \ data.len();\n        data.insert(0, alg.unit());\n        for i in 1..=len {\n\
    \            if i + lsb(i) <= len {\n                data[i + lsb(i)] = alg.op(data[i\
    \ + lsb(i)], data[i]);\n            }\n        }\n        Self { data, alg }\n\
    \    }\n    pub fn len(&self) -> usize {\n        self.data.len() - 1\n    }\n\
    \    pub fn clear(&mut self) {\n        for e in &mut self.data {\n          \
    \  *e = self.alg.unit();\n        }\n    }\n    pub fn add(&mut self, pos: usize,\
    \ v: T) {\n        let mut pos = pos + 1;\n        while pos < self.data.len()\
    \ {\n            self.data[pos] = self.alg.op(self.data[pos], v);\n          \
    \  pos += lsb(pos);\n        }\n    }\n    pub fn push(&mut self, v: T) {\n  \
    \      self.data.push(self.alg.unit());\n        self.add(self.data.len() - 1,\
    \ v);\n    }\n    pub fn ask_prefix(&self, mut r: usize) -> T {\n        let mut\
    \ res = self.alg.unit();\n        while r != 0 {\n            res = self.alg.op(self.data[r],\
    \ res);\n            r -= lsb(r);\n        }\n        res\n    }\n    pub fn partition_point<F:\
    \ FnMut(T) -> bool>(&self, mut pred: F) -> usize {\n        let mut x = 0; //\
    \ pred(&self.ask_prefix(x)) == true\n        let mut w = self.data.len().next_power_of_two()\
    \ >> 1;\n        let mut l = self.alg.unit();\n        while w != 0 {\n      \
    \      if x + w < self.data.len() && pred(self.alg.op(l, self.data[x + w])) {\n\
    \                x += w;\n                l = self.alg.op(l, self.data[x + w]);\n\
    \            }\n            w >>= 1;\n        }\n        x + 1\n    }\n    pub\
    \ fn lower_bound(&self, v: T) -> usize\n    where\n        T: Ord,\n    {\n  \
    \      self.partition_point(|x| x < v)\n    }\n    pub fn upper_bound(&self, v:\
    \ T) -> usize\n    where\n        T: Ord,\n    {\n        self.partition_point(|x|\
    \ x <= v)\n    }\n}\n\nimpl<T: Copy, M: Group<T>> FenwickTree<T, M> {\n    pub\
    \ fn sub(&mut self, pos: usize, v: T) {\n        self.add(pos, self.alg.inv(v));\n\
    \    }\n    pub fn ask(&self, l: usize, r: usize) -> T {\n        self.alg.op(self.alg.inv(self.ask_prefix(l)),\
    \ self.ask_prefix(r))\n    }\n}\n\nfn lsb(n: usize) -> usize {\n    n & (!n +\
    \ 1)\n}\n"
  dependsOn:
  - src/alg.rs
  - src/alg/arith.rs
  - src/num.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/ds/fenwick.rs
  requiredBy: []
  timestamp: '2021-02-20 14:04:23+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/tree_dfs_io_test.rs
documentation_of: src/ds/fenwick.rs
layout: document
redirect_from:
- /library/src/ds/fenwick.rs
- /library/src/ds/fenwick.rs.html
title: src/ds/fenwick.rs
---
