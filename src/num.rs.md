---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: src/zo.rs
    title: src/zo.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: src/alg/arith.rs
    title: src/alg/arith.rs
  - icon: ':x:'
    path: src/dfa.rs
    title: src/dfa.rs
  - icon: ':warning:'
    path: src/draft/fpacc64.rs
    title: src/draft/fpacc64.rs
  - icon: ':warning:'
    path: src/draft/graph/path.rs
    title: src/draft/graph/path.rs
  - icon: ':warning:'
    path: src/ds/fenwick.rs
    title: src/ds/fenwick.rs
  - icon: ':warning:'
    path: src/fp/num.rs
    title: src/fp/num.rs
  - icon: ':warning:'
    path: src/graph/dijkstra.rs
    title: src/graph/dijkstra.rs
  - icon: ':warning:'
    path: src/graph/euler_tour.rs
    title: src/graph/euler_tour.rs
  - icon: ':x:'
    path: src/int.rs
    title: src/int.rs
  - icon: ':warning:'
    path: src/int/arith.rs
    title: src/int/arith.rs
  - icon: ':warning:'
    path: src/int/bisect.rs
    title: src/int/bisect.rs
  - icon: ':x:'
    path: src/int/gcd.rs
    title: src/int/gcd.rs
  - icon: ':warning:'
    path: src/int/inv.rs
    title: src/int/inv.rs
  - icon: ':warning:'
    path: src/math/binom.rs
    title: src/math/binom.rs
  - icon: ':warning:'
    path: src/math/pow.rs
    title: src/math/pow.rs
  - icon: ':warning:'
    path: src/mint/num.rs
    title: src/mint/num.rs
  - icon: ':warning:'
    path: src/slice/cum.rs
    title: src/slice/cum.rs
  - icon: ':x:'
    path: src/tests.rs
    title: src/tests.rs
  _extendedVerifiedWith:
  - icon: ':x:'
    path: test/src/bin/cargo_test.rs
    title: test/src/bin/cargo_test.rs
  - icon: ':x:'
    path: test/src/bin/dfa_test.rs
    title: test/src/bin/dfa_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/lazy_segtree_test.rs
    title: test/src/bin/lazy_segtree_test.rs
  _pathExtension: rs
  _verificationStatusIcon: ':question:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/num.rs\n"
  code: "pub use crate::zo::ZeroOne;\nuse std::fmt::*;\nuse std::ops::*;\n\npub trait\
    \ Num:\n\tZeroOne\n\t+ Add<Output = Self>\n\t+ AddAssign\n\t+ Sub<Output = Self>\n\
    \t+ SubAssign\n\t+ Mul<Output = Self>\n\t+ MulAssign\n\t+ Div<Output = Self>\n\
    \t+ DivAssign\n\t+ Debug\n\t+ Display\n{\n\tfn wrapping_add(self, rhs: Self) ->\
    \ Self;\n\tfn wrapping_neg(self) -> Self;\n}\n\npub trait INum: Num + Neg<Output\
    \ = Self> {}\n"
  dependsOn:
  - src/zo.rs
  isVerificationFile: false
  path: src/num.rs
  requiredBy:
  - src/slice/cum.rs
  - src/fp/num.rs
  - src/ds/fenwick.rs
  - src/tests.rs
  - src/int.rs
  - src/graph/dijkstra.rs
  - src/graph/euler_tour.rs
  - src/alg/arith.rs
  - src/int/bisect.rs
  - src/int/inv.rs
  - src/int/gcd.rs
  - src/int/arith.rs
  - src/mint/num.rs
  - src/dfa.rs
  - src/math/pow.rs
  - src/math/binom.rs
  - src/draft/fpacc64.rs
  - src/draft/graph/path.rs
  timestamp: '2021-01-27 17:46:37+09:00'
  verificationStatus: LIBRARY_SOME_WA
  verifiedWith:
  - test/src/bin/dfa_test.rs
  - test/src/bin/cargo_test.rs
  - test/src/bin/lazy_segtree_test.rs
documentation_of: src/num.rs
layout: document
redirect_from:
- /library/src/num.rs
- /library/src/num.rs.html
title: src/num.rs
---
