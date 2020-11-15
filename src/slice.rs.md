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
  code: "pub trait Slice {\n    type Item;\n    fn fill(&mut self, value: Self::Item)\n\
    \    where\n        Self::Item: Clone;\n    /// min { i | !pred(arr[i]) }\n  \
    \  fn partition_point<F: FnMut(&Self::Item) -> bool>(&self, pred: F) -> usize;\n\
    \    fn lower_bound(&self, v: &Self::Item) -> usize\n    where\n        Self::Item:\
    \ Ord,\n    {\n        self.partition_point(|x| x < v)\n    }\n    fn upper_bound(&self,\
    \ v: &Self::Item) -> usize\n    where\n        Self::Item: Ord,\n    {\n     \
    \   self.partition_point(|x| x <= v)\n    }\n}\n\nimpl<T> Slice for [T] {\n  \
    \  type Item = T;\n    fn fill(&mut self, value: Self::Item)\n    where\n    \
    \    Self::Item: Clone,\n    {\n        self.iter_mut().for_each(|e| e.clone_from(&value));\n\
    \    }\n    fn partition_point<F: FnMut(&Self::Item) -> bool>(&self, mut pred:\
    \ F) -> usize {\n        let (mut lb, mut ub) = (0, self.len()); // pred(self[ub])\
    \ == false\n        while lb != ub {\n            let mid = (lb + ub) / 2;\n \
    \           if pred(&self[mid]) {\n                lb = mid + 1;\n           \
    \ } else {\n                ub = mid;\n            }\n        }\n        ub\n\
    \    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/slice.rs
  requiredBy:
  - src/lib.rs
  timestamp: '2020-11-16 03:39:01+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/slice.rs
layout: document
redirect_from:
- /library/src/slice.rs
- /library/src/slice.rs.html
title: src/slice.rs
---
