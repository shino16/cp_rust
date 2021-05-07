---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/zo.rs
    title: src/zo.rs
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
    RuntimeError: bundler is not specified: src/slice/cum.rs\n"
  code: "use crate::zo::*;\nuse std::ops::Add;\n\npub fn cuml<T: Copy>(slice: &[T],\
    \ zero: T, mut op: impl FnMut(T, T) -> T) -> Vec<T> {\n    let mut res = Vec::with_capacity(slice.len()\
    \ + 1);\n    let mut tl = zero;\n    res.push(tl);\n    for &e in slice {\n  \
    \      tl = op(tl, e);\n        res.push(tl);\n    }\n    res\n}\n\npub fn cumr<T:\
    \ Copy>(slice: &[T], zero: T, mut op: impl FnMut(T, T) -> T) -> Vec<T> {\n   \
    \ let mut res = Vec::with_capacity(slice.len() + 1);\n    let mut tl = zero;\n\
    \    res.push(tl);\n    for &e in slice.iter().rev() {\n        tl = op(e, tl);\n\
    \        res.push(tl);\n    }\n    res.reverse();\n    res\n}\n\npub fn cuml_sum<T:\
    \ Copy + ZeroOne + Add<Output = T>>(slice: &[T]) -> Vec<T> {\n    cuml(slice,\
    \ T::ZERO, Add::add)\n}\n\npub fn cumr_sum<T: Copy + ZeroOne + Add<Output = T>>(slice:\
    \ &[T]) -> Vec<T> {\n    cumr(slice, T::ZERO, Add::add)\n}\n"
  dependsOn:
  - src/zo.rs
  isVerificationFile: false
  path: src/slice/cum.rs
  requiredBy: []
  timestamp: '2021-05-04 17:50:45+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/slice/cum.rs
layout: document
redirect_from:
- /library/src/slice/cum.rs
- /library/src/slice/cum.rs.html
title: src/slice/cum.rs
---
