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
  code: "pub mod lazy;\npub use crate::alg::*;\nuse std::ops::Index;\nuse std::slice::SliceIndex;\n\
    \n#[derive(Clone)]\npub struct SegmentTree<A: Alg> {\n\tlen: usize,\n\tdata: Vec<A::Item>,\n\
    \talg: A,\n}\n\nimpl<A: Monoid> SegmentTree<A> {\n\tpub fn new(len: usize, alg:\
    \ A) -> Self {\n\t\tSelf {\n\t\t\tlen,\n\t\t\tdata: vec![alg.unit(); len * 2],\n\
    \t\t\talg,\n\t\t}\n\t}\n\tpub fn new_from_slice(slice: &[A::Item], alg: A) ->\
    \ Self {\n\t\tlet len = slice.len();\n\t\tlet mut data = slice.to_vec();\n\t\t\
    data.extend_from_slice(slice);\n\t\tfor i in (1..len).rev() {\n\t\t\tdata[i] =\
    \ alg.op(data[i << 1], data[i << 1 | 1]);\n\t\t}\n\t\tSelf { len, data, alg }\n\
    \t}\n\tpub fn len(&self) -> usize {\n\t\tself.len\n\t}\n\tfn build(&mut self,\
    \ mut p: usize) {\n\t\tp >>= 1;\n\t\twhile p != 0 {\n\t\t\tself.data[p] = self.alg.op(self.data[p\
    \ << 1], self.data[p << 1 | 1]);\n\t\t\tp >>= 1;\n\t\t}\n\t}\n\tpub fn add(&mut\
    \ self, pos: usize, v: A::Item) {\n\t\tlet p = pos + self.len();\n\t\tself.data[p]\
    \ = self.alg.op(self.data[p], v);\n\t\tself.build(p);\n\t}\n\tpub fn exec<F: FnOnce(&mut\
    \ A::Item)>(&mut self, pos: usize, f: F) {\n\t\tlet p = pos + self.len();\n\t\t\
    f(&mut self.data[p]);\n\t\tself.build(p);\n\t}\n\tpub fn ask(&self, mut l: usize,\
    \ mut r: usize) -> A::Item {\n\t\tlet (mut resl, mut resr) = (self.alg.unit(),\
    \ self.alg.unit());\n\t\tl += self.len();\n\t\tr += self.len();\n\t\twhile l <\
    \ r {\n\t\t\tif l & 1 != 0 {\n\t\t\t\tresl = self.alg.op(resl, self.data[l]);\n\
    \t\t\t\tl += 1;\n\t\t\t}\n\t\t\tif r & 1 != 0 {\n\t\t\t\tresr = self.alg.op(self.data[r\
    \ - 1], resr);\n\t\t\t\tr -= 1;\n\t\t\t}\n\t\t\tl >>= 1;\n\t\t\tr >>= 1;\n\t\t\
    }\n\t\tself.alg.op(resl, resr)\n\t}\n}\n\nimpl<A: Monoid, I: SliceIndex<[A::Item]>>\
    \ Index<I> for SegmentTree<A> {\n    type Output = I::Output;\n    fn index(&self,\
    \ idx: I) -> &Self::Output {\n\t\t&self.data[self.len()..][idx]\n    }\n}\n"
  dependsOn:
  - src/alg.rs
  isVerificationFile: false
  path: src/ds/segtree.rs
  requiredBy: []
  timestamp: '2021-01-30 17:33:56+09:00'
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
