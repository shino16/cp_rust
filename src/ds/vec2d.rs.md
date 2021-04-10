---
data:
  _extendedDependsOn: []
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
    RuntimeError: bundler is not specified: src/ds/vec2d.rs\n"
  code: "use std::ops::{Index, IndexMut};\n\n#[derive(Clone)]\npub struct Vec2D<T>\
    \ {\n    pub h: usize,\n    pub w: usize,\n    pub inner: Vec<T>,\n}\n\nimpl<T:\
    \ Clone> Vec2D<T> {\n    pub fn fill(h: usize, w: usize, v: T) -> Self {\n   \
    \     Self { h, w, inner: vec![v; h * w] }\n    }\n    pub fn resize_from(h: usize,\
    \ w: usize, inner: Vec<T>) -> Self {\n        debug_assert_eq!(inner.len(), h\
    \ * w);\n        Self { h, w, inner }\n    }\n}\n\nimpl<T> Index<[usize; 2]> for\
    \ Vec2D<T> {\n    type Output = T;\n    fn index(&self, [r, c]: [usize; 2]) ->\
    \ &Self::Output {\n        &self.inner[r * self.w + c]\n    }\n}\n\nimpl<T> IndexMut<[usize;\
    \ 2]> for Vec2D<T> {\n    fn index_mut(&mut self, [r, c]: [usize; 2]) -> &mut\
    \ Self::Output {\n        &mut self.inner[r * self.w + c]\n    }\n}\n\nimpl<T>\
    \ Index<usize> for Vec2D<T> {\n    type Output = [T];\n    fn index(&self, r:\
    \ usize) -> &Self::Output {\n        &self.inner[r * self.w..(r + 1) * self.w]\n\
    \    }\n}\n\nimpl<T> IndexMut<usize> for Vec2D<T> {\n    fn index_mut(&mut self,\
    \ r: usize) -> &mut Self::Output {\n        &mut self.inner[r * self.w..(r + 1)\
    \ * self.w]\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/ds/vec2d.rs
  requiredBy: []
  timestamp: '2021-02-08 00:55:24+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/ds/vec2d.rs
layout: document
redirect_from:
- /library/src/ds/vec2d.rs
- /library/src/ds/vec2d.rs.html
title: src/ds/vec2d.rs
---
