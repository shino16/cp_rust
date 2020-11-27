---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/segtree_test.rs
    title: test/src/bin/segtree_test.rs
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "pub trait Alg {\n\ttype Item: Clone;\n}\n\npub trait Monoid: Alg {\n\tfn\
    \ unit(&self) -> Self::Item;\n\tfn op(&self, x: &Self::Item, y: &Self::Item) ->\
    \ Self::Item;\n}\n\npub trait Group: Monoid {\n\tfn inv(&self, x: &Self::Item)\
    \ -> Self::Item;\n}\n\nmacro_rules! impl_monoid {\n\t($target:ty, $($params:tt\
    \ : $bounds:tt),*) => {\n\t\timpl<$($params : $bounds),*> Alg for $target {\n\t\
    \t\ttype Item = T;\n\t\t}\n\t\timpl<$($params : $bounds),*> Monoid for $target\
    \ {\n\t\t\tfn unit(&self) -> Self::Item {\n\t\t\t\t(self.0)()\n\t\t\t}\n\t\t\t\
    fn op(&self, x: &Self::Item, y: &Self::Item) -> Self::Item {\n\t\t\t\t(self.1)(x,\
    \ y)\n\t\t\t}\n\t\t}\n\t};\n}\n\nmacro_rules! impl_group {\n\t($target:ty, $($params:tt\
    \ : $bounds:tt),*) => {\n\t\timpl_monoid!($target, $($params : $bounds),*);\n\t\
    \timpl<$($params : $bounds),*> Group for $target {\n\t\t\tfn inv(&self, x: &Self::Item)\
    \ -> Self::Item {\n\t\t\t\t(self.2)(x)\n\t\t\t}\n\t\t}\n\t};\n}\n\npub struct\
    \ MonoidImpl<T: Clone, Unit, Op>(pub Unit, pub Op)\nwhere\n\tT: Clone,\n\tUnit:\
    \ Fn() -> T,\n\tOp: Fn(&T, &T) -> T;\npub struct GroupImpl<T, Unit, Op, Inv>(pub\
    \ Unit, pub Op, pub Inv)\nwhere\n\tT: Clone,\n\tUnit: Fn() -> T,\n\tOp: Fn(&T,\
    \ &T) -> T,\n\tInv: Fn(&T) -> T;\n\n// help!\nimpl_monoid!(MonoidImpl<T, Unit,\
    \ Op>, T: Clone, Unit: (Fn() -> T), Op: (Fn(&T, &T) -> T));\nimpl_group!(GroupImpl<T,\
    \ Unit, Op, Inv>,\n\t\t\tT: Clone, Unit: (Fn() -> T), Op: (Fn(&T, &T) -> T), Inv:\
    \ (Fn(&T) -> T));\n"
  dependsOn: []
  isVerificationFile: false
  path: src/alg.rs
  requiredBy: []
  timestamp: '2020-11-27 14:24:44+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/segtree_test.rs
documentation_of: src/alg.rs
layout: document
redirect_from:
- /library/src/alg.rs
- /library/src/alg.rs.html
title: src/alg.rs
---
