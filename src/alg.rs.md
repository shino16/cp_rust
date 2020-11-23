---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':x:'
    path: test/src/bin/dfa_test.rs
    title: test/src/bin/dfa_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/segtree_test.rs
    title: test/src/bin/segtree_test.rs
  _pathExtension: rs
  _verificationStatusIcon: ':question:'
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
    \        }\n    };\n}\n\npub struct MonoidImpl<T: Clone, Unit, Op>(pub Unit, pub\
    \ Op)\nwhere\n    T: Clone,\n    Unit: Fn() -> T,\n    Op: Fn(&T, &T) -> T;\n\
    pub struct GroupImpl<T, Unit, Op, Inv>(pub Unit, pub Op, pub Inv)\nwhere\n   \
    \ T: Clone,\n    Unit: Fn() -> T,\n    Op: Fn(&T, &T) -> T,\n    Inv: Fn(&T) ->\
    \ T;\n\n// help!\nimpl_monoid!(MonoidImpl<T, Unit, Op>, T: Clone, Unit: (Fn()\
    \ -> T), Op: (Fn(&T, &T) -> T));\nimpl_group!(GroupImpl<T, Unit, Op, Inv>,\n \
    \           T: Clone, Unit: (Fn() -> T), Op: (Fn(&T, &T) -> T), Inv: (Fn(&T) ->\
    \ T));\n"
  dependsOn: []
  isVerificationFile: false
  path: src/alg.rs
  requiredBy: []
  timestamp: '2020-11-18 23:11:02+09:00'
  verificationStatus: LIBRARY_SOME_WA
  verifiedWith:
  - test/src/bin/segtree_test.rs
  - test/src/bin/dfa_test.rs
documentation_of: src/alg.rs
layout: document
redirect_from:
- /library/src/alg.rs
- /library/src/alg.rs.html
title: src/alg.rs
---
