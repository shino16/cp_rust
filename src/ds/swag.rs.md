---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/alg.rs
    title: src/alg.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/swag_test.rs
    title: test/src/bin/swag_test.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/ds/swag.rs\n"
  code: "pub use crate::alg::*;\n\npub struct Swag<T, M> {\n    front: Vec<(T, T)>,\n\
    \    front_prod: T,\n    back: Vec<T>,\n    back_prod: T,\n    alg: M,\n}\n\n\
    impl<T: Copy, M: Monoid<T>> Swag<T, M> {\n    pub fn new(alg: M) -> Self {\n \
    \       Self {\n            front: Vec::new(),\n            front_prod: alg.unit(),\n\
    \            back: Vec::new(),\n            back_prod: alg.unit(),\n         \
    \   alg,\n        }\n    }\n    pub fn ask(&mut self) -> T {\n        self.ensure_front();\n\
    \        self.alg.op(self.front_prod, self.back_prod)\n    }\n    pub fn push(&mut\
    \ self, elt: T) {\n        self.back.push(elt);\n        self.alg.op_to(elt, &mut\
    \ self.back_prod);\n    }\n    pub fn extend<I: IntoIterator<Item = T>>(&mut self,\
    \ into_iter: I) {\n        let iter = into_iter.into_iter();\n        self.back.reserve(iter.size_hint().0);\n\
    \        for elt in iter {\n            self.push(elt);\n        }\n    }\n  \
    \  pub fn extend_from_slice(&mut self, slice: &[T]) {\n        self.back.extend_from_slice(slice);\n\
    \        for &elt in &self.back[self.back.len() - slice.len()..] {\n         \
    \   self.alg.op_to(elt, &mut self.back_prod);\n        }\n    }\n    pub fn pop(&mut\
    \ self) -> T {\n        self.ensure_front();\n        let (elt, prod) = self.front.pop().unwrap();\n\
    \        self.front_prod = prod;\n        elt\n    }\n    fn ensure_front(&mut\
    \ self) {\n        if !self.front.is_empty() {\n            return;\n        }\n\
    \        self.front.reserve(self.back.len());\n        let mut prod = self.alg.unit();\n\
    \        while let Some(elt) = self.back.pop() {\n            self.front.push((elt,\
    \ prod));\n            self.alg.op_to(elt, &mut prod);\n        }\n        self.front_prod\
    \ = prod;\n        self.back_prod = self.alg.unit();\n    }\n}\n"
  dependsOn:
  - src/alg.rs
  isVerificationFile: false
  path: src/ds/swag.rs
  requiredBy: []
  timestamp: '2021-02-20 13:37:47+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/swag_test.rs
documentation_of: src/ds/swag.rs
layout: document
redirect_from:
- /library/src/ds/swag.rs
- /library/src/ds/swag.rs.html
title: src/ds/swag.rs
---
