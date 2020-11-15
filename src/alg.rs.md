---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':warning:'
    path: src/lib.rs
    title: src/lib.rs
  _extendedVerifiedWith: []
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "pub trait Alg {\n    type Item: Clone;\n}\n\npub trait Monoid: Alg {\n  \
    \  fn unit(&self) -> Self::Item;\n    fn op(&self, x: &Self::Item, y: &Self::Item)\
    \ -> Self::Item;\n}\n\npub trait Group: Monoid {\n    fn inv(&self, x: &Self::Item)\
    \ -> Self::Item;\n}\n\nmacro_rules! impl_monoid {\n    ($target:ty, $($params:tt\
    \ : $bounds:tt),*) => {\n        impl<$($params : $bounds),*> Alg for $target\
    \ {\n            type Item = T;\n        }\n        impl<$($params : $bounds),*>\
    \ Monoid for $target {\n            fn unit(&self) -> Self::Item {\n         \
    \       (self.0)()\n            }\n            fn op(&self, x: &Self::Item, y:\
    \ &Self::Item) -> Self::Item {\n                (self.1)(x, y)\n            }\n\
    \        }\n    };\n}\n\nmacro_rules! impl_group {\n    ($target:ty, $($params:tt\
    \ : $bounds:tt),*) => {\n        impl_monoid!($target, $($params : $bounds),*);\n\
    \        impl<$($params : $bounds),*> Group for $target {\n            fn inv(&self,\
    \ x: &Self::Item) -> Self::Item {\n                (self.2)(x)\n            }\n\
    \        }\n    };\n}\n\npub struct MonoidImpl<Unit, Op>(pub Unit, pub Op);\n\
    pub struct GroupImpl<Unit, Op, Inv>(pub Unit, pub Op, pub Inv);\n\n// help!\n\
    impl_monoid!(MonoidImpl<Unit, Op>, T: Clone, Unit: (Fn() -> T), Op: (Fn(&T, &T)\
    \ -> T));\nimpl_group!(GroupImpl<Unit, Op, Inv>,\n            T: Clone, Unit:\
    \ (Fn() -> T), Op: (Fn(&T, &T) -> T), Inv: (Fn(&T) -> T));\n"
  dependsOn: []
  isVerificationFile: false
  path: src/alg.rs
  requiredBy:
  - src/lib.rs
  timestamp: '2020-11-01 20:07:08+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/alg.rs
layout: document
redirect_from:
- /library/src/alg.rs
- /library/src/alg.rs.html
title: src/alg.rs
---
