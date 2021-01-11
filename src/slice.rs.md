---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':warning:'
    path: src/hash.rs
    title: src/hash.rs
  _extendedVerifiedWith: []
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/slice.rs\n"
  code: "pub mod cum;\npub mod perm;\n\npub trait Slice {\n\ttype Item;\n\tfn fill(&mut\
    \ self, value: Self::Item)\n\twhere\n\t\tSelf::Item: Clone;\n\t/// min { i | !pred(arr[i])\
    \ }\n\tfn partition_point<F: FnMut(&Self::Item) -> bool>(&self, pred: F) -> usize;\n\
    \tfn lower_bound(&self, v: &Self::Item) -> usize\n\twhere\n\t\tSelf::Item: Ord,\n\
    \t{\n\t\tself.partition_point(|x| x < v)\n\t}\n\tfn upper_bound(&self, v: &Self::Item)\
    \ -> usize\n\twhere\n\t\tSelf::Item: Ord,\n\t{\n\t\tself.partition_point(|x| x\
    \ <= v)\n\t}\n}\n\nimpl<T> Slice for [T] {\n\ttype Item = T;\n\tfn fill(&mut self,\
    \ value: Self::Item)\n\twhere\n\t\tSelf::Item: Clone,\n\t{\n\t\tself.iter_mut().for_each(|e|\
    \ e.clone_from(&value));\n\t}\n\tfn partition_point<F: FnMut(&Self::Item) -> bool>(&self,\
    \ mut pred: F) -> usize {\n\t\tlet (mut l, mut r) = (0, self.len()); // pred(self[r])\
    \ == false\n\t\twhile l != r {\n\t\t\tlet mid = (l + r) / 2;\n\t\t\tlet val =\
    \ unsafe { self.get_unchecked(mid) };\n\t\t\tif pred(val) {\n\t\t\t\tl = mid +\
    \ 1;\n\t\t\t} else {\n\t\t\t\tr = mid;\n\t\t\t}\n\t\t}\n\t\tr\n\t}\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/slice.rs
  requiredBy:
  - src/hash.rs
  timestamp: '2021-01-12 01:50:14+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/slice.rs
layout: document
redirect_from:
- /library/src/slice.rs
- /library/src/slice.rs.html
title: src/slice.rs
---
