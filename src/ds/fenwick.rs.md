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
    path: src/bit.rs
    title: src/bit.rs
  - icon: ':heavy_check_mark:'
    path: src/num.rs
    title: src/num.rs
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
    RuntimeError: bundler is not specified: src/ds/fenwick.rs\n"
  code: "pub use crate::alg::arith::*;\nuse crate::bit::*;\n\n#[derive(Clone)]\npub\
    \ struct FenwickTree<A: Alg> {\n\tdata: Vec<A::Item>,\n\talg: A,\n}\n\n/// A:\
    \ Commutative\nimpl<A: Monoid> FenwickTree<A> {\n\tpub fn new(mut data: Vec<A::Item>,\
    \ alg: A) -> Self {\n\t\tlet len = data.len();\n\t\tdata.insert(0, alg.unit());\n\
    \t\tfor i in 1..=len {\n\t\t\tif i + i.lsb() <= len {\n\t\t\t\tdata[i + i.lsb()]\
    \ = alg.op(data[i + i.lsb()], data[i]);\n\t\t\t}\n\t\t}\n\t\tSelf { data, alg\
    \ }\n\t}\n\tpub fn len(&self) -> usize {\n\t\tself.data.len() - 1\n\t}\n\tpub\
    \ fn add(&mut self, pos: usize, v: A::Item) {\n\t\tlet mut pos = pos + 1;\n\t\t\
    while pos < self.data.len() {\n\t\t\tself.data[pos] = self.alg.op(self.data[pos],\
    \ v);\n\t\t\tpos += pos.lsb();\n\t\t}\n\t}\n\tpub fn push(&mut self, v: A::Item)\
    \ {\n\t\tself.data.push(self.alg.unit());\n\t\tself.add(self.data.len() - 1, v);\n\
    \t}\n\tpub fn ask_prefix(&self, mut r: usize) -> A::Item {\n\t\tlet mut res =\
    \ self.alg.unit();\n\t\twhile r != 0 {\n\t\t\tres = self.alg.op(self.data[r],\
    \ res);\n\t\t\tr -= r.lsb();\n\t\t}\n\t\tres\n\t}\n\tpub fn partition_point<F:\
    \ FnMut(A::Item) -> bool>(&self, mut pred: F) -> usize {\n\t\tlet mut x = 0; //\
    \ pred(&self.ask_prefix(x)) == true\n\t\tlet mut w = (self.data.len() - 1).msb();\n\
    \t\tlet mut l = self.alg.unit();\n\t\twhile w != 0 {\n\t\t\tif x + w < self.data.len()\
    \ && pred(self.alg.op(l, self.data[x + w])) {\n\t\t\t\tx += w;\n\t\t\t\tl = self.alg.op(l,\
    \ self.data[x + w]);\n\t\t\t}\n\t\t\tw >>= 1;\n\t\t}\n\t\tx + 1\n\t}\n\tpub fn\
    \ lower_bound(&self, v: A::Item) -> usize\n\twhere\n\t\tA::Item: Ord,\n\t{\n\t\
    \tself.partition_point(|x| x < v)\n\t}\n\tpub fn upper_bound(&self, v: A::Item)\
    \ -> usize\n\twhere\n\t\tA::Item: Ord,\n\t{\n\t\tself.partition_point(|x| x <=\
    \ v)\n\t}\n}\n\n/// A: Commutative\nimpl<A: Group> FenwickTree<A> {\n\tpub fn\
    \ ask(&self, l: usize, r: usize) -> A::Item {\n\t\tself.alg.op(self.alg.inv(self.ask_prefix(l)),\
    \ self.ask_prefix(r))\n\t}\n}\n"
  dependsOn:
  - src/alg.rs
  - src/alg/arith.rs
  - src/bit.rs
  - src/num.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/ds/fenwick.rs
  requiredBy: []
  timestamp: '2021-01-27 17:46:37+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/ds/fenwick.rs
layout: document
redirect_from:
- /library/src/ds/fenwick.rs
- /library/src/ds/fenwick.rs.html
title: src/ds/fenwick.rs
---
