---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: src/alg.rs
    title: src/alg.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':x:'
    path: test/src/bin/lazy_segtree_test.rs
    title: test/src/bin/lazy_segtree_test.rs
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':x:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/ds/segtree/lazy.rs\n"
  code: "pub use crate::alg::*;\n\nfn trunc(x: usize) -> usize {\n    x >> x.trailing_zeros()\n\
    }\n\n#[derive(Clone)]\npub struct LazySegmentTree<On: Monoid, Act: Monoid, Apply>\n\
    where\n    Apply: Fn(On::Item, Act::Item) -> On::Item,\n{\n    len: usize,\n \
    \   log: u32,\n    data: Vec<(On::Item, Act::Item)>,\n    on_alg: On,\n    act_alg:\
    \ Act,\n    apply: Apply,\n}\n\nimpl<On: Monoid, Act: Monoid, Apply: Fn(On::Item,\
    \ Act::Item) -> On::Item>\n    LazySegmentTree<On, Act, Apply>\n{\n    pub fn\
    \ new(len: usize, on_alg: On, act_alg: Act, apply: Apply) -> Self {\n        Self\
    \ {\n            len,\n            log: len.next_power_of_two().trailing_zeros(),\n\
    \            data: vec![(on_alg.unit(), act_alg.unit()); len * 2],\n         \
    \   on_alg,\n            act_alg,\n            apply,\n        }\n    }\n    pub\
    \ fn from_slice(slice: &[On::Item], on_alg: On, act_alg: Act, apply: Apply) ->\
    \ Self {\n        let len = slice.len();\n        let log = len.next_power_of_two().trailing_zeros();\n\
    \        let iter = slice.iter().map(|&x| (x, act_alg.unit()));\n        let mut\
    \ data: Vec<_> = iter.clone().chain(iter).collect();\n        for i in (1..len).rev()\
    \ {\n            data[i].0 = on_alg.op(data[i << 1].0, data[i << 1 | 1].0);\n\
    \        }\n        Self { len, log, data, on_alg, act_alg, apply }\n    }\n \
    \   pub fn len(&self) -> usize {\n        self.len\n    }\n    fn apply(&mut self,\
    \ p: usize, actor: Act::Item) {\n        self.data[p].0 = (self.apply)(self.data[p].0,\
    \ actor);\n        self.act_alg.op_to(actor, &mut self.data[p].1);\n    }\n  \
    \  fn flush(&mut self, p: usize) {\n        for s in (1..=self.log).rev() {\n\
    \            let p = p >> s;\n            self.apply(p << 1, self.data[p].1);\n\
    \            self.apply(p << 1 | 1, self.data[p].1);\n            self.data[p].1\
    \ = self.act_alg.unit();\n        }\n    }\n    fn build(&mut self, mut p: usize)\
    \ {\n        p >>= 1;\n        while p != 0 {\n            self.data[p].0 = self.on_alg.op(self.data[p\
    \ << 1].0, self.data[p << 1 | 1].0);\n            // debug_assert_eq!(self.data[p].1,\
    \ self.act_alg.unit());\n            p >>= 1;\n        }\n    }\n    pub fn ask(&mut\
    \ self, l: usize, r: usize) -> On::Item {\n        self.flush(trunc(l + self.len()));\n\
    \        self.flush(trunc(r + self.len()) - 1);\n        let [mut resl, mut resr]\
    \ = [self.on_alg.unit(); 2];\n        let (mut l, mut r) = (l + self.len(), r\
    \ + self.len());\n        while l < r {\n            if l & 1 != 0 {\n       \
    \         resl = self.on_alg.op(resl, self.data[l].0);\n                l += 1;\n\
    \            }\n            if r & 1 != 0 {\n                r -= 1;\n       \
    \         resr = self.on_alg.op(self.data[r].0, resr);\n            }\n      \
    \      l >>= 1;\n            r >>= 1;\n        }\n        self.on_alg.op(resl,\
    \ resr)\n    }\n    pub fn exec<F: FnOnce(&mut On::Item)>(&mut self, pos: usize,\
    \ f: F) {\n        self.flush(pos + self.len());\n        let p = pos + self.len();\n\
    \        f(&mut self.data[p].0);\n        self.build(trunc(pos + self.len()));\n\
    \    }\n    pub fn act_over(&mut self, l: usize, r: usize, actor: Act::Item) {\n\
    \        self.flush(trunc(l + self.len()));\n        self.flush(trunc(r + self.len())\
    \ - 1);\n        {\n            let (mut l, mut r) = (l + self.len(), r + self.len());\n\
    \            while l < r {\n                if l & 1 != 0 {\n                \
    \    self.apply(l, actor);\n                    l += 1;\n                }\n \
    \               if r & 1 != 0 {\n                    r -= 1;\n               \
    \     self.apply(r, actor);\n                }\n                l >>= 1;\n   \
    \             r >>= 1;\n            }\n        }\n        self.build(trunc(l +\
    \ self.len()));\n        self.build(trunc(r + self.len()) - 1);\n    }\n}\n"
  dependsOn:
  - src/alg.rs
  isVerificationFile: false
  path: src/ds/segtree/lazy.rs
  requiredBy: []
  timestamp: '2021-02-08 00:55:24+09:00'
  verificationStatus: LIBRARY_ALL_WA
  verifiedWith:
  - test/src/bin/lazy_segtree_test.rs
documentation_of: src/ds/segtree/lazy.rs
layout: document
redirect_from:
- /library/src/ds/segtree/lazy.rs
- /library/src/ds/segtree/lazy.rs.html
title: src/ds/segtree/lazy.rs
---
