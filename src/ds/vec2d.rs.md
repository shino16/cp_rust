---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':warning:'
    path: src/lib.rs
    title: src/lib.rs
  _extendedVerifiedWith: []
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "use std::ops::{Index, IndexMut};\n\n#[derive(Clone)]\npub struct Vec2D<T>\
    \ {\n    pub h: usize,\n    pub w: usize,\n    pub inner: Vec<T>,\n}\n\nimpl<T:\
    \ Clone> Vec2D<T> {\n    pub fn fill(h: usize, w: usize, v: T) -> Self {\n   \
    \     Self { h, w, inner: vec![v; h * w] }\n    }\n    pub fn resize_from(h: usize,\
    \ w: usize, inner: Vec<T>) -> Self {\n        debug_assert_eq!(inner.len(), h\
    \ * w);\n        Self { h, w, inner }\n    }\n}\n\nimpl<T> Index<(usize, usize)>\
    \ for Vec2D<T> {\n    type Output = T;\n    fn index(&self, (r, c): (usize, usize))\
    \ -> &Self::Output {\n        &self.inner[r * self.w + c]\n    }\n}\n\nimpl<T>\
    \ IndexMut<(usize, usize)> for Vec2D<T> {\n    fn index_mut(&mut self, (r, c):\
    \ (usize, usize)) -> &mut Self::Output {\n        &mut self.inner[r * self.w +\
    \ c]\n    }\n}\n\nimpl<T> Index<usize> for Vec2D<T> {\n    type Output = [T];\n\
    \    fn index(&self, r: usize) -> &Self::Output {\n        &self.inner[r * self.w..(r\
    \ + 1) * self.w]\n    }\n}\n\nimpl<T> IndexMut<usize> for Vec2D<T> {\n    fn index_mut(&mut\
    \ self, r: usize) -> &mut Self::Output {\n        &mut self.inner[r * self.w..(r\
    \ + 1) * self.w]\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/ds/vec2d.rs
  requiredBy:
  - src/lib.rs
  timestamp: '2020-11-16 03:39:01+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/ds/vec2d.rs
layout: document
redirect_from:
- /library/src/ds/vec2d.rs
- /library/src/ds/vec2d.rs.html
title: src/ds/vec2d.rs
---
