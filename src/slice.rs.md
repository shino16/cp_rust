---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':warning:'
    path: src/comp.rs
    title: src/comp.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.4/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.4/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/slice.rs\n"
  code: "pub mod cum;\npub mod fill;\npub mod lcp;\npub mod perm;\npub mod rle;\n\
    pub mod sa;\npub mod sort;\n\npub fn partition_point<T>(slice: &[T], mut pred:\
    \ impl FnMut(&T) -> bool) -> usize {\n    let (mut l, mut r) = (0, slice.len());\
    \ // pred(slice[r]) == false\n    while l != r {\n        let mid = (l + r) /\
    \ 2;\n        let val = unsafe { slice.get_unchecked(mid) };\n        if pred(val)\
    \ {\n            l = mid + 1;\n        } else {\n            r = mid;\n      \
    \  }\n    }\n    r\n}\n\npub fn lower_bound<T: Ord>(slice: &[T], v: &T) -> usize\
    \ {\n    partition_point(slice, |x| x < v)\n}\n\npub fn upper_bound<T: Ord>(slice:\
    \ &[T], v: &T) -> usize {\n    partition_point(slice, |x| x <= v)\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/slice.rs
  requiredBy:
  - src/comp.rs
  timestamp: '2021-03-31 15:51:17+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/slice.rs
layout: document
redirect_from:
- /library/src/slice.rs
- /library/src/slice.rs.html
title: src/slice.rs
---
