---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/bounded.rs
    title: src/bounded.rs
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
    RuntimeError: bundler is not specified: src/int.rs\n"
  code: "pub use crate::bounded::*;\nuse crate::cast::*;\npub use crate::num::*;\n\
    pub use crate::zo::*;\nuse std::ops::*;\n\npub mod arith;\npub mod bisect;\npub\
    \ mod gcd;\npub mod inv;\npub mod saturate;\n\npub trait Int: Num + Ord + Rem<Output\
    \ = Self> + RemAssign + Bounded + PrimCast {\n    type Signed: IInt + CastFrom<Self>\
    \ + CastTo<Self>;\n    type Unsigned: UInt + CastFrom<Self> + CastTo<Self>;\n\
    \    fn abs(self) -> Self::Unsigned;\n    fn rem_euclid(self, rhs: Self::Unsigned)\
    \ -> Self::Unsigned;\n}\n\npub trait IInt: Int + INum {}\npub trait UInt: Int\
    \ {}\n\nmacro_rules! impl_int {\n    (@ $t:ident, $i:ident, $u:ident, $abs:expr)\
    \ => {\n        impl Int for $t {\n            type Signed = $i;\n           \
    \ type Unsigned = $u;\n            fn abs(self) -> Self::Unsigned {\n        \
    \        $abs(self) as $u\n            }\n            fn rem_euclid(self, rhs:\
    \ Self::Unsigned) -> Self::Unsigned {\n                <$t>::rem_euclid(self,\
    \ rhs as $t) as $u\n            }\n        }\n    };\n    ({ $i:ident }, { $u:ident\
    \ }) => {\n        impl_int!(@ $i, $i, $u, |x| <$i>::abs(x));\n        impl_int!(@\
    \ $u, $i, $u, |x| x);\n        impl IInt for $i {}\n        impl UInt for $u {}\n\
    \    };\n    ({ $i:ident, $($is:ident),* }, { $u:ident, $($us:ident),* }) => {\n\
    \        impl_int!({ $i }, { $u });\n        impl_int!({ $($is),* }, { $($us),*\
    \ });\n    }\n}\n\nimpl_int!({ i32, i64, i128, isize }, { u32, u64, u128, usize\
    \ });\n"
  dependsOn:
  - src/bounded.rs
  - src/cast.rs
  - src/num.rs
  - src/util/trait_alias.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/int.rs
  requiredBy:
  - src/math/pow.rs
  - src/graph/dijkstra.rs
  - src/draft/fpacc64.rs
  - src/int/bisect.rs
  - src/int/inv.rs
  - src/int/gcd.rs
  - src/int/gcd/ext.rs
  - src/int/arith.rs
  - src/dfa.rs
  - src/tests.rs
  timestamp: '2021-02-24 00:44:23+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/segtree_beats_test.rs
  - test/src/bin/cargo_test.rs
  - test/src/bin/dfa_test.rs
documentation_of: src/int.rs
layout: document
redirect_from:
- /library/src/int.rs
- /library/src/int.rs.html
title: src/int.rs
---
