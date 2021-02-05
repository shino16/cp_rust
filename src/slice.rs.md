---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':warning:'
    path: src/hash.rs
    title: src/hash.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/slice.rs\n"
  code: "pub mod cum;\npub mod fill;\npub mod perm;\npub mod sa;\npub mod sort;\n\n\
    pub fn partition_point<T, F: FnMut(&T) -> bool>(slice: &[T], mut pred: F) -> usize\
    \ {\n\tlet (mut l, mut r) = (0, slice.len()); // pred(slice[r]) == false\n\twhile\
    \ l != r {\n\t\tlet mid = (l + r) / 2;\n\t\tlet val = unsafe { slice.get_unchecked(mid)\
    \ };\n\t\tif pred(val) {\n\t\t\tl = mid + 1;\n\t\t} else {\n\t\t\tr = mid;\n\t\
    \t}\n\t}\n\tr\n}\n\npub fn lower_bound<T: Ord>(slice: &[T], v: &T) -> usize {\n\
    \tpartition_point(slice, |x| x < v)\n}\n\npub fn upper_bound<T: Ord>(slice: &[T],\
    \ v: &T) -> usize {\n\tpartition_point(slice, |x| x <= v)\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/slice.rs
  requiredBy:
  - src/hash.rs
  timestamp: '2021-02-06 00:52:06+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/slice.rs
layout: document
redirect_from:
- /library/src/slice.rs
- /library/src/slice.rs.html
title: src/slice.rs
---
