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
  - icon: ':heavy_check_mark:'
    path: src/int.rs
    title: src/int.rs
  - icon: ':warning:'
    path: src/int/arith.rs
    title: src/int/arith.rs
  - icon: ':warning:'
    path: src/int/bisect.rs
    title: src/int/bisect.rs
  - icon: ':heavy_check_mark:'
    path: src/int/gcd.rs
    title: src/int/gcd.rs
  - icon: ':warning:'
    path: src/int/gcd/ext.rs
    title: src/int/gcd/ext.rs
  - icon: ':warning:'
    path: src/int/inv.rs
    title: src/int/inv.rs
  - icon: ':warning:'
    path: src/math/pow.rs
    title: src/math/pow.rs
  - icon: ':heavy_check_mark:'
    path: src/tests.rs
    title: src/tests.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/cargo_test.rs
    title: test/src/bin/cargo_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/dfa_test.rs
    title: test/src/bin/dfa_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/segtree_beats_test.rs
    title: test/src/bin/segtree_beats_test.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/cast.rs\n"
  code: "pub trait CastTo<T> {\n    fn cast_to(self) -> T;\n}\npub trait CastFrom<T>\
    \ {\n    fn cast_from(src: T) -> Self;\n}\n\nimpl<T, U: CastTo<T>> CastFrom<U>\
    \ for T {\n    fn cast_from(src: U) -> Self {\n        U::cast_to(src)\n    }\n\
    }\n\nmacro_rules! impl_prim {\n    ($($ts:ty),*) => {\n        impl_asint!({ $($ts),*\
    \ } => { $($ts),* });\n        pub trait PrimCast where $(Self: CastTo<$ts>),*,\
    \ $(Self: CastFrom<$ts>),* {}\n        $( impl PrimCast for $ts {} )*\n    }\n\
    }\n\nmacro_rules! impl_asint {\n    ({ $t:ty } => { $($us:ty),* }) => { $(\n \
    \       impl CastTo<$us> for $t {\n            fn cast_to(self) -> $us {\n   \
    \             self as $us\n            }\n        }\n    )* };\n    ({ $t:ty,\
    \ $($ts:ty),* } => { $($us:ty),* }) => {\n        impl_asint!({ $t } => { $($us),*\
    \ });\n        impl_asint!({ $($ts),* } => { $($us),* });\n    };\n}\n\nimpl_prim!(i32,\
    \ i64, i128, isize, u32, u64, u128, usize, f32, f64);\n\npub trait As: Sized {\n\
    \    fn as_<T: CastFrom<Self>>(self) -> T {\n        T::cast_from(self)\n    }\n\
    \    fn into_<T: From<Self>>(self) -> T {\n        T::from(self)\n    }\n}\n\n\
    impl<T> As for T {}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/cast.rs
  requiredBy:
  - src/draft/fpacc64.rs
  - src/dfa.rs
  - src/graph/dijkstra.rs
  - src/math/pow.rs
  - src/tests.rs
  - src/int.rs
  - src/int/bisect.rs
  - src/int/arith.rs
  - src/int/inv.rs
  - src/int/gcd/ext.rs
  - src/int/gcd.rs
  timestamp: '2021-02-08 00:55:24+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/cargo_test.rs
  - test/src/bin/segtree_beats_test.rs
  - test/src/bin/dfa_test.rs
documentation_of: src/cast.rs
layout: document
redirect_from:
- /library/src/cast.rs
- /library/src/cast.rs.html
title: src/cast.rs
---
