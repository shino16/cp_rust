---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: src/draft/linked_list.rs
    title: src/draft/linked_list.rs
  - icon: ':warning:'
    path: src/ds.rs
    title: src/ds.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/draft/linked_list/iter.rs\n"
  code: "pub use super::*;\nuse std::marker::PhantomData;\n\npub struct Iter<'a, T:\
    \ 'a> {\n\thead: NonNull<Node<T>>,\n\ttail: NonNull<Node<T>>,\n\tlen: usize,\n\
    \t_marker: PhantomData<&'a Node<T>>,\n}\n\nimpl<'a, T: 'a> Iterator for Iter<'a,\
    \ T> {\n    type Item = &'a T;\n    fn next(&mut self) -> Option<Self::Item> {\n\
    \t\tlet next_val = unsafe { &*self.head.as_ptr() }.next_val.as_ref()?;\n\t\tlet\
    \ res = &next_val.1;\n\t\tself.head = next_val.0;\n\t\tSome(res)\n\t}\n\tfn size_hint(&self)\
    \ -> (usize, Option<usize>) {\n\t\t(self.len, Some(self.len))\n\t}\n}\n\nimpl<'a,\
    \ T: 'a> DoubleEndedIterator for Iter<'a, T> {\n    fn next_back(&mut self) ->\
    \ Option<Self::Item> {\n\t\tself.tail = unsafe { self.tail.as_ref() }.prev?;\n\
    \t\tSome(&unsafe { &*self.tail.as_ptr() }.next_val.as_ref().unwrap().1)\n    }\n\
    }\n"
  dependsOn:
  - src/draft/linked_list.rs
  - src/ds.rs
  isVerificationFile: false
  path: src/draft/linked_list/iter.rs
  requiredBy: []
  timestamp: '2021-01-30 17:33:56+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/draft/linked_list/iter.rs
layout: document
redirect_from:
- /library/src/draft/linked_list/iter.rs
- /library/src/draft/linked_list/iter.rs.html
title: src/draft/linked_list/iter.rs
---
