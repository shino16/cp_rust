---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: src/bit.rs
    title: src/bit.rs
  - icon: ':question:'
    path: src/cast.rs
    title: src/cast.rs
  - icon: ':question:'
    path: src/int.rs
    title: src/int.rs
  - icon: ':question:'
    path: src/zo.rs
    title: src/zo.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/math/binom.rs\n"
  code: "use crate::int::*;\nuse crate::cast::*;\n\npub struct Binom<T> {\n\tpub fact:\
    \ Vec<T>,\n\tpub inv_fact: Vec<T>,\n}\n\nimpl<I: Num + CastFrom<usize>> Binom<I>\
    \ {\n\tpub fn new(n: usize) -> Self {\n\t\tlet mut fact = Vec::with_capacity(n\
    \ + 1);\n\t\tlet mut inv_fact = Vec::with_capacity(n + 1);\n\t\tlet n: I = n.as_();\n\
    \t\tlet (mut acc, mut now) = (I::ONE, I::ZERO);\n\t\tfact.push(I::ONE);\n\t\t\
    while now != n {\n\t\t\tnow += I::ONE;\n\t\t\tacc *= now;\n\t\t\tfact.push(acc);\n\
    \t\t}\n\t\tacc = I::ONE / acc;\n\t\twhile now != I::ZERO {\n\t\t\tinv_fact.push(acc);\n\
    \t\t\tacc *= now;\n\t\t\tnow -= I::ONE;\n\t\t}\n\t\tinv_fact.push(I::ONE);\n\t\
    \tinv_fact.reverse();\n\t\tSelf { fact, inv_fact }\n\t}\n\tpub fn binom<J: CastTo<usize>>(&self,\
    \ n: J, r: J) -> I {\n\t\tlet [n, r]: [usize; 2] = [n.as_(), r.as_()];\n\t\tself.fact[n]\
    \ * self.inv_fact[r] * self.inv_fact[n - r]\n\t}\n}\n"
  dependsOn:
  - src/bit.rs
  - src/cast.rs
  - src/int.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/math/binom.rs
  requiredBy: []
  timestamp: '2021-01-03 22:19:51+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/math/binom.rs
layout: document
redirect_from:
- /library/src/math/binom.rs
- /library/src/math/binom.rs.html
title: src/math/binom.rs
---
