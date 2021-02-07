---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/bit.rs
    title: src/bit.rs
  - icon: ':heavy_check_mark:'
    path: src/bounded.rs
    title: src/bounded.rs
  - icon: ':heavy_check_mark:'
    path: src/cast.rs
    title: src/cast.rs
  - icon: ':heavy_check_mark:'
    path: src/int.rs
    title: src/int.rs
  - icon: ':question:'
    path: src/num.rs
    title: src/num.rs
  - icon: ':question:'
    path: src/zo.rs
    title: src/zo.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/math/binom.rs\n"
  code: "use crate::int::*;\nuse crate::cast::*;\n\npub struct Binom<T> {\n    pub\
    \ fact: Vec<T>,\n    pub inv_fact: Vec<T>,\n}\n\nimpl<I: Num + CastFrom<usize>>\
    \ Binom<I> {\n    pub fn new(n: usize) -> Self {\n        let mut fact = Vec::with_capacity(n\
    \ + 1);\n        let mut inv_fact = Vec::with_capacity(n + 1);\n        let n:\
    \ I = n.as_();\n        let (mut acc, mut now) = (I::ONE, I::ZERO);\n        fact.push(I::ONE);\n\
    \        while now != n {\n            now += I::ONE;\n            acc *= now;\n\
    \            fact.push(acc);\n        }\n        acc = I::ONE / acc;\n       \
    \ while now != I::ZERO {\n            inv_fact.push(acc);\n            acc *=\
    \ now;\n            now -= I::ONE;\n        }\n        inv_fact.push(I::ONE);\n\
    \        inv_fact.reverse();\n        Self { fact, inv_fact }\n    }\n    pub\
    \ fn binom<J: CastTo<usize>>(&self, n: J, r: J) -> I {\n        let [n, r]: [usize;\
    \ 2] = [n.as_(), r.as_()];\n        self.fact[n] * self.inv_fact[r] * self.inv_fact[n\
    \ - r]\n    }\n}\n"
  dependsOn:
  - src/bit.rs
  - src/bounded.rs
  - src/cast.rs
  - src/int.rs
  - src/num.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/math/binom.rs
  requiredBy: []
  timestamp: '2021-02-08 00:55:24+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/math/binom.rs
layout: document
redirect_from:
- /library/src/math/binom.rs
- /library/src/math/binom.rs.html
title: src/math/binom.rs
---
