---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/num.rs
    title: src/num.rs
  - icon: ':heavy_check_mark:'
    path: src/util/trait_alias.rs
    title: src/util/trait_alias.rs
  - icon: ':heavy_check_mark:'
    path: src/zo.rs
    title: src/zo.rs
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
    RuntimeError: bundler is not specified: src/math/binom.rs\n"
  code: "use crate::num::*;\n\npub struct Binom<T> {\n    pub fact: Vec<T>,\n    pub\
    \ inv_fact: Vec<T>,\n}\n\nimpl<I: Num + From<usize>> Binom<I> {\n    pub fn new(n:\
    \ usize) -> Self {\n        let mut fact = Vec::with_capacity(n + 1);\n      \
    \  let mut inv_fact = Vec::with_capacity(n + 1);\n        let n: I = n.into();\n\
    \        let (mut acc, mut now) = (I::ONE, I::ZERO);\n        fact.push(I::ONE);\n\
    \        while now != n {\n            now += I::ONE;\n            acc *= now;\n\
    \            fact.push(acc);\n        }\n        acc = I::ONE / acc;\n       \
    \ while now != I::ZERO {\n            inv_fact.push(acc);\n            acc *=\
    \ now;\n            now -= I::ONE;\n        }\n        inv_fact.push(I::ONE);\n\
    \        inv_fact.reverse();\n        Self { fact, inv_fact }\n    }\n    pub\
    \ fn binom(&self, n: usize, r: usize) -> I {\n        if n < r {\n           \
    \ I::ZERO\n        } else {\n            self.fact[n] * self.inv_fact[r] * self.inv_fact[n\
    \ - r]\n        }\n    }\n}\n"
  dependsOn:
  - src/num.rs
  - src/util/trait_alias.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/math/binom.rs
  requiredBy: []
  timestamp: '2021-03-31 15:51:17+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/math/binom.rs
layout: document
redirect_from:
- /library/src/math/binom.rs
- /library/src/math/binom.rs.html
title: src/math/binom.rs
---
