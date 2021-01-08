---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: src/graph.rs
    title: src/graph.rs
  - icon: ':heavy_check_mark:'
    path: src/zo.rs
    title: src/zo.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/draft/graph/ds.rs\n"
  code: "pub use crate::graph::*;\nuse std::marker::PhantomData;\nuse std::ops::{Index,\
    \ IndexMut};\n\npub type Dict<'a, V, T> = <V as MkDict<'a, T>>::Type;\n\npub trait\
    \ MkDict<'a, T>: Sized {\n\ttype Type: Index<Self, Output = T> + IndexMut<Self>\
    \ + 'a;\n\tfn new(self, v: T) -> Self::Type;\n}\n\n// impl Dim for usize {}\n\
    impl<'a, T: Clone + 'a> MkDict<'a, T> for usize {\n\ttype Type = Vec<T>;\n\tfn\
    \ new(self, v: T) -> Self::Type {\n\t\tvec![v; self]\n\t}\n}\n\n// impl<A, B>\
    \ Dim for (A, B) {}\nimpl<'a, T: Clone + 'a, A: MkDict<'a, B::Type>, B: MkDict<'a,\
    \ T>> MkDict<'a, T> for (A, B) {\n\ttype Type = DWrap<'a, A::Type, T>;\n\tfn new(self,\
    \ v: T) -> Self::Type {\n\t\tDWrap(self.0.new(self.1.new(v)), PhantomData)\n\t\
    }\n}\n\npub struct DWrap<'a, D, T>(D, PhantomData<&'a T>);\n\nimpl<'a, T, A: MkDict<'a,\
    \ B::Type>, B: MkDict<'a, T>> Index<(A, B)> for DWrap<'a, A::Type, T> {\n\ttype\
    \ Output = T;\n\tfn index(&self, (a, b): (A, B)) -> &Self::Output {\n\t\t&self.0[a][b]\n\
    \t}\n}\n\nimpl<'a, T, A: MkDict<'a, B::Type>, B: MkDict<'a, T>> IndexMut<(A, B)>\n\
    \tfor DWrap<'a, A::Type, T>\n{\n\tfn index_mut(&mut self, (a, b): (A, B)) -> &mut\
    \ Self::Output {\n\t\t&mut self.0[a][b]\n\t}\n}\n"
  dependsOn:
  - src/graph.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/draft/graph/ds.rs
  requiredBy: []
  timestamp: '2021-01-03 22:19:51+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/draft/graph/ds.rs
layout: document
redirect_from:
- /library/src/draft/graph/ds.rs
- /library/src/draft/graph/ds.rs.html
title: src/draft/graph/ds.rs
---
