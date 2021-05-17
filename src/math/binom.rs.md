---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/cast.rs
    title: src/cast.rs
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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.5/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.5/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/math/binom.rs\n"
  code: "use crate::num::*;\nuse crate::cast::*;\n\npub struct Binom<T> {\n    pub\
    \ fact: Vec<T>,\n    pub inv_fact: Vec<T>,\n}\n\nimpl<T: Num + From<usize>> Binom<T>\
    \ {\n    pub fn new<I: PrimInt>(n: I) -> Self {\n        let n: usize = n.cast();\n\
    \        let mut fact = Vec::with_capacity(n + 1);\n        let mut inv_fact =\
    \ Vec::with_capacity(n + 1);\n        let n: T = n.into();\n        let (mut acc,\
    \ mut now) = (T::one(), T::zero());\n        fact.push(T::one());\n        while\
    \ now != n {\n            now += T::one();\n            acc *= now;\n        \
    \    fact.push(acc);\n        }\n        acc = T::one() / acc;\n        while\
    \ now != T::zero() {\n            inv_fact.push(acc);\n            acc *= now;\n\
    \            now -= T::one();\n        }\n        inv_fact.push(T::one());\n \
    \       inv_fact.reverse();\n        Self { fact, inv_fact }\n    }\n    pub fn\
    \ call<I: PrimInt>(&self, n: I, r: I) -> T {\n        let (n, r): (usize, usize)\
    \ = (n.cast(), r.cast());\n        if n < r {\n            T::zero()\n       \
    \ } else {\n            self.fact[n] * self.inv_fact[r] * self.inv_fact[n - r]\n\
    \        }\n    }\n}\n"
  dependsOn:
  - src/cast.rs
  - src/num.rs
  - src/util/trait_alias.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/math/binom.rs
  requiredBy: []
  timestamp: '2021-05-17 15:14:26+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/math/binom.rs
layout: document
redirect_from:
- /library/src/math/binom.rs
- /library/src/math/binom.rs.html
title: src/math/binom.rs
---
