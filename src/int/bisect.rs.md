---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/cargo_test.rs
    title: test/src/bin/cargo_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/dfa_test.rs
    title: test/src/bin/dfa_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/ntt_garner_test.rs
    title: test/src/bin/ntt_garner_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/ntt_mint_garner_test.rs
    title: test/src/bin/ntt_mint_garner_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/ntt_mint_test.rs
    title: test/src/bin/ntt_mint_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/ntt_test.rs
    title: test/src/bin/ntt_test.rs
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "use crate::int::*;\n\npub fn bisect<I: Int, F: FnMut(I) -> bool>(mut l: I,\
    \ mut r: I, mut pred: F) -> I {\n\twhile l != r {\n\t\tlet mid = (l + r) >> 1;\n\
    \t\tif pred(mid) {\n\t\t\tl = mid + I::ONE;\n\t\t} else {\n\t\t\tr = mid;\n\t\t\
    }\n\t}\n\tr\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/int/bisect.rs
  requiredBy: []
  timestamp: '2020-12-10 17:35:58+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/ntt_garner_test.rs
  - test/src/bin/dfa_test.rs
  - test/src/bin/ntt_test.rs
  - test/src/bin/ntt_mint_test.rs
  - test/src/bin/cargo_test.rs
  - test/src/bin/ntt_mint_garner_test.rs
documentation_of: src/int/bisect.rs
layout: document
redirect_from:
- /library/src/int/bisect.rs
- /library/src/int/bisect.rs.html
title: src/int/bisect.rs
---
