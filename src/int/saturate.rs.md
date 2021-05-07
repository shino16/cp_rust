---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.5/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.5/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/int/saturate.rs\n"
  code: "use std::ops::*;\n\n#[repr(transparent)]\n#[derive(Debug, Clone, Copy, PartialEq,\
    \ Eq, PartialOrd, Ord, Default, Hash)]\npub struct Saturate<T>(pub T);\n\nimpl<T>\
    \ From<T> for Saturate<T> { fn from(x: T) -> Self { Self(x) } }\n\nmacro_rules!\
    \ impl_ops {\n    ($($t:ty),*) => { $(\n        impl<T: Into<Saturate<$t>>> Add<T>\
    \ for Saturate<$t> {\n            type Output = Self;\n            fn add(self,\
    \ rhs: T) -> Self { Self(self.0.saturating_add(rhs.into().0)) }\n        }\n \
    \       impl<T: Into<Saturate<$t>>> AddAssign<T> for Saturate<$t> {\n        \
    \    fn add_assign(&mut self, rhs: T) { *self = *self + rhs; }\n        }\n  \
    \  )* };\n}\n\nimpl_ops!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128,\
    \ usize);\n"
  dependsOn: []
  isVerificationFile: false
  path: src/int/saturate.rs
  requiredBy: []
  timestamp: '2021-02-20 13:28:01+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/int/saturate.rs
layout: document
redirect_from:
- /library/src/int/saturate.rs
- /library/src/int/saturate.rs.html
title: src/int/saturate.rs
---
