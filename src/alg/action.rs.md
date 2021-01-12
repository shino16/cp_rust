---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/alg.rs
    title: src/alg.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/alg/action.rs\n"
  code: "pub use super::*;\n\npub trait Action {\n\ttype Actor: Monoid;\n\ttype On:\
    \ Monoid;\n\tfn act(\n\t\t&self,\n\t\ton: <Self::On as Alg>::Item,\n\t\tactor:\
    \ <Self::Actor as Alg>::Item,\n\t) -> <Self::On as Alg>::Item;\n}\n\npub struct\
    \ ActionImpl<On: Monoid, A: Monoid, F: Fn(On::Item, A::Item) -> On::Item>(\n\t\
    pub On,\n\tpub A,\n\tpub F,\n);\n\nimpl<On: Monoid, A: Monoid, F: Fn(On::Item,\
    \ A::Item) -> On::Item> Action\n\tfor ActionImpl<On, A, F>\n{\n\ttype Actor =\
    \ A;\n\ttype On = On;\n\tfn act(&self, on: On::Item, actor: A::Item) -> On::Item\
    \ {\n\t\tself.2(on, actor)\n\t}\n}\n"
  dependsOn:
  - src/alg.rs
  isVerificationFile: false
  path: src/alg/action.rs
  requiredBy: []
  timestamp: '2021-01-03 22:19:36+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/alg/action.rs
layout: document
redirect_from:
- /library/src/alg/action.rs
- /library/src/alg/action.rs.html
title: src/alg/action.rs
---
