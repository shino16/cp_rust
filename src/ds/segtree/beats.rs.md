---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/alg.rs
    title: src/alg.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/segtree_beats_test.rs
    title: test/src/bin/segtree_beats_test.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/ds/segtree/beats.rs\n"
  code: "pub use crate::alg::*;\n\nfn trunc(x: usize) -> usize {\n    x >> x.trailing_zeros()\n\
    }\n\n#[derive(Clone)]\npub struct SegmentTreeBeats<T, A, MT, MA, Apply>\nwhere\n\
    \    Apply: Fn(T, A) -> Option<T>,\n{\n    len: usize,\n    log: u32,\n    data:\
    \ Vec<(T, A)>,\n    on_alg: MT,\n    act_alg: MA,\n    apply: Apply,\n}\n\nimpl<T:\
    \ Copy, A: Copy, MT: Monoid<T>, MA: Monoid<A>, Apply: Fn(T, A) -> Option<T>>\n\
    \    SegmentTreeBeats<T, A, MT, MA, Apply>\n{\n    pub fn new(len: usize, on_alg:\
    \ MT, act_alg: MA, apply: Apply) -> Self {\n        Self {\n            len,\n\
    \            log: len.next_power_of_two().trailing_zeros(),\n            data:\
    \ vec![(on_alg.unit(), act_alg.unit()); len * 2],\n            on_alg,\n     \
    \       act_alg,\n            apply,\n        }\n    }\n    pub fn from_slice(slice:\
    \ &[T], on_alg: MT, act_alg: MA, apply: Apply) -> Self {\n        let len = slice.len();\n\
    \        let log = len.next_power_of_two().trailing_zeros();\n        let iter\
    \ = slice.iter().map(|&x| (x, act_alg.unit()));\n        let mut data: Vec<_>\
    \ = iter.clone().chain(iter).collect();\n        for i in (1..len).rev() {\n \
    \           data[i].0 = on_alg.op(data[i << 1].0, data[i << 1 | 1].0);\n     \
    \   }\n        Self { len, log, data, on_alg, act_alg, apply }\n    }\n    pub\
    \ fn len(&self) -> usize {\n        self.len\n    }\n    fn apply(&mut self, p:\
    \ usize, actor: A) {\n        self.act_alg.op_to(actor, &mut self.data[p].1);\n\
    \        self.data[p].0 = if let Some(d) = (self.apply)(self.data[p].0, actor)\
    \ {\n            d\n        } else {\n            self.push(p);\n            self.on_alg.op(self.data[p\
    \ << 1].0, self.data[p << 1 | 1].0)\n        };\n    }\n    fn push(&mut self,\
    \ p: usize) {\n        self.apply(p << 1, self.data[p].1);\n        self.apply(p\
    \ << 1 | 1, self.data[p].1);\n        self.data[p].1 = self.act_alg.unit();\n\
    \    }\n    fn flush(&mut self, p: usize) {\n        for s in (1..=self.log).rev()\
    \ {\n            self.push(p >> s);\n        }\n    }\n    fn build(&mut self,\
    \ mut p: usize) {\n        p >>= 1;\n        while p != 0 {\n            self.data[p].0\
    \ = self.on_alg.op(self.data[p << 1].0, self.data[p << 1 | 1].0);\n          \
    \  // debug_assert_eq!(self.data[p].1, self.act_alg.unit());\n            p >>=\
    \ 1;\n        }\n    }\n    pub fn ask(&mut self, l: usize, r: usize) -> T {\n\
    \        self.flush(trunc(l + self.len()));\n        self.flush(trunc(r + self.len())\
    \ - 1);\n        let [mut resl, mut resr] = [self.on_alg.unit(); 2];\n       \
    \ let (mut l, mut r) = (l + self.len(), r + self.len());\n        while l < r\
    \ {\n            if l & 1 != 0 {\n                resl = self.on_alg.op(resl,\
    \ self.data[l].0);\n                l += 1;\n            }\n            if r &\
    \ 1 != 0 {\n                r -= 1;\n                resr = self.on_alg.op(self.data[r].0,\
    \ resr);\n            }\n            l >>= 1;\n            r >>= 1;\n        }\n\
    \        self.on_alg.op(resl, resr)\n    }\n    pub fn with<F: FnOnce(&mut T)\
    \ -> R, R>(&mut self, pos: usize, f: F) -> R {\n        self.flush(pos + self.len());\n\
    \        let p = pos + self.len();\n        let r = f(&mut self.data[p].0);\n\
    \        self.build(pos + self.len());\n        r\n    }\n    pub fn act_over(&mut\
    \ self, l: usize, r: usize, actor: A) {\n        self.flush(trunc(l + self.len()));\n\
    \        self.flush(trunc(r + self.len()) - 1);\n        {\n            let (mut\
    \ l, mut r) = (l + self.len(), r + self.len());\n            while l < r {\n \
    \               if l & 1 != 0 {\n                    self.apply(l, actor);\n \
    \                   l += 1;\n                }\n                if r & 1 != 0\
    \ {\n                    r -= 1;\n                    self.apply(r, actor);\n\
    \                }\n                l >>= 1;\n                r >>= 1;\n     \
    \       }\n        }\n        self.build(trunc(l + self.len()));\n        self.build(trunc(r\
    \ + self.len()) - 1);\n    }\n}\n"
  dependsOn:
  - src/alg.rs
  isVerificationFile: false
  path: src/ds/segtree/beats.rs
  requiredBy: []
  timestamp: '2021-02-21 16:57:52+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/segtree_beats_test.rs
documentation_of: src/ds/segtree/beats.rs
layout: document
redirect_from:
- /library/src/ds/segtree/beats.rs
- /library/src/ds/segtree/beats.rs.html
title: src/ds/segtree/beats.rs
---