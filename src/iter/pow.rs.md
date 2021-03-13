---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/iter/pow.rs\n"
  code: "/// pow_iter(0..10, 8): 700 ms (AtC) / 900 ms (CF)\npub fn pow_iter<I: Iterator\
    \ + Clone>(iter: I, k: usize) -> IterPow<I> {\n    IterPow {\n        iters: vec![iter.clone();\
    \ k],\n        iters0: vec![iter; k],\n        state: Vec::with_capacity(k),\n\
    \    }\n}\n\npub struct IterPow<I: Iterator + Clone> {\n    iters: Vec<I>,\n \
    \   iters0: Vec<I>,\n    state: Vec<I::Item>,\n}\n\nimpl<'a, I: Iterator + Clone>\
    \ IterPow<I>\nwhere\n    I::Item: Clone,\n{\n    pub fn next(&mut self) -> Option<&Vec<I::Item>>\
    \ {\n        if self.state.is_empty() {\n            for iter in self.iters.iter_mut()\
    \ {\n                self.state.push(iter.next()?);\n            }\n         \
    \   return Some(&self.state);\n        }\n        for ((iter, iter0), state) in\n\
    \            self.iters.iter_mut().zip(self.iters0.iter()).zip(self.state.iter_mut())\n\
    \        {\n            if let Some(e) = iter.next() {\n                *state\
    \ = e;\n                return Some(&self.state);\n            }\n           \
    \ *iter = iter0.clone();\n        }\n        None\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/iter/pow.rs
  requiredBy: []
  timestamp: '2021-02-08 00:55:24+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/iter/pow.rs
layout: document
redirect_from:
- /library/src/iter/pow.rs
- /library/src/iter/pow.rs.html
title: src/iter/pow.rs
---
