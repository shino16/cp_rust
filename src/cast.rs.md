---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: src/dfa.rs
    title: src/dfa.rs
  - icon: ':warning:'
    path: src/draft/fpacc64.rs
    title: src/draft/fpacc64.rs
  - icon: ':warning:'
    path: src/graph/dijkstra.rs
    title: src/graph/dijkstra.rs
  - icon: ':question:'
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
  - icon: ':x:'
    path: src/tests.rs
    title: src/tests.rs
  _extendedVerifiedWith:
  - icon: ':x:'
    path: test/src/bin/cargo_test.rs
    title: test/src/bin/cargo_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/dfa_test.rs
    title: test/src/bin/dfa_test.rs
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':question:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/cast.rs\n"
  code: "pub trait CastTo<T> {\n\tfn cast_to(self) -> T;\n}\npub trait CastFrom<T>\
    \ {\n\tfn cast_from(src: T) -> Self;\n}\n\nimpl<T, U: CastTo<T>> CastFrom<U> for\
    \ T {\n\tfn cast_from(src: U) -> Self {\n\t\tU::cast_to(src)\n\t}\n}\n\nmacro_rules!\
    \ impl_prim {\n\t($($ts:ty),*) => {\n\t\timpl_asint!({ $($ts),* } => { $($ts),*\
    \ });\n\t\tpub trait PrimCast where $(Self: CastTo<$ts>),*, $(Self: CastFrom<$ts>),*\
    \ {}\n\t\t$( impl PrimCast for $ts {} )*\n\t}\n}\n\nmacro_rules! impl_asint {\n\
    \t({ $t:ty } => { $($us:ty),* }) => { $(\n\t\timpl CastTo<$us> for $t {\n\t\t\t\
    fn cast_to(self) -> $us {\n\t\t\t\tself as $us\n\t\t\t}\n\t\t}\n\t)* };\n\t({\
    \ $t:ty, $($ts:ty),* } => { $($us:ty),* }) => {\n\t\timpl_asint!({ $t } => { $($us),*\
    \ });\n\t\timpl_asint!({ $($ts),* } => { $($us),* });\n\t};\n}\n\nimpl_prim!(i32,\
    \ i64, i128, isize, u32, u64, u128, usize, f32, f64);\n\npub trait As: Sized {\n\
    \tfn as_<T: CastFrom<Self>>(self) -> T {\n\t\tT::cast_from(self)\n\t}\n\tfn into_<T:\
    \ From<Self>>(self) -> T {\n\t\tT::from(self)\n\t}\n}\n\nimpl<T> As for T {}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/cast.rs
  requiredBy:
  - src/draft/fpacc64.rs
  - src/int/inv.rs
  - src/int/bisect.rs
  - src/int/gcd.rs
  - src/int/arith.rs
  - src/graph/dijkstra.rs
  - src/math/binom.rs
  - src/math/pow.rs
  - src/int.rs
  - src/dfa.rs
  - src/tests.rs
  timestamp: '2020-12-21 16:30:24+09:00'
  verificationStatus: LIBRARY_SOME_WA
  verifiedWith:
  - test/src/bin/cargo_test.rs
  - test/src/bin/dfa_test.rs
documentation_of: src/cast.rs
layout: document
redirect_from:
- /library/src/cast.rs
- /library/src/cast.rs.html
title: src/cast.rs
---
