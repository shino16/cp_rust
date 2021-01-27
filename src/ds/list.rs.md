---
data:
  _extendedDependsOn: []
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
    RuntimeError: bundler is not specified: src/ds/list.rs\n"
  code: "use std::iter::FromIterator;\nuse std::rc::Rc;\n\n#[derive(Clone, Ord, PartialOrd,\
    \ Eq, PartialEq, Debug)]\npub struct List<T>(Option<Rc<(T, List<T>)>>);\n\nimpl<T>\
    \ Default for List<T> {\n\tfn default() -> Self {\n\t\tSelf::nil()\n\t}\n}\n\n\
    impl<T> List<T> {\n\tpub fn nil() -> Self {\n\t\tSelf(None)\n\t}\n\tpub fn push(&mut\
    \ self, val: T) {\n\t\tlet me = std::mem::take(self);\n\t\t*self = Self(Some(Rc::new((val,\
    \ me))));\n\t}\n\tpub fn cons(self, val: T) -> Self {\n\t\tSelf(Some(Rc::new((val,\
    \ self))))\n\t}\n\tpub fn head(&self) -> Option<&T> {\n\t\tself.0.as_deref().map(|(hd,\
    \ _)| hd)\n\t}\n\tpub fn tail(&self) -> Option<&Self> {\n\t\tself.0.as_deref().map(|(_,\
    \ tl)| tl)\n\t}\n\tpub fn from_rev_iter<I: IntoIterator<Item = T>>(iter: I) ->\
    \ Self {\n\t\titer.into_iter().fold(Self::nil(), |es, e| es.cons(e))\n\t}\n}\n\
    \npub struct Iter<'a, T>(&'a List<T>);\n\nimpl<'a, T> Iterator for Iter<'a, T>\
    \ {\n\ttype Item = &'a T;\n\tfn next(&mut self) -> Option<Self::Item> {\n\t\t\
    match (self.0).0.as_deref() {\n\t\t\tNone => None,\n\t\t\tSome(&(ref hd, ref tl))\
    \ => {\n\t\t\t\t*self = Self(tl);\n\t\t\t\tSome(hd)\n\t\t\t},\n\t\t}\n\t}\n}\n\
    \nimpl<T> FromIterator<T> for List<T> {\n\tfn from_iter<I: IntoIterator<Item =\
    \ T>>(iter: I) -> Self {\n\t\tlet mut iter = iter.into_iter();\n\t\tlet val =\
    \ iter.next();\n\t\tSelf(val.map(|val| Rc::new((val, Self::from_iter(iter)))))\n\
    \t}\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/ds/list.rs
  requiredBy: []
  timestamp: '2020-12-10 17:35:58+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/ds/list.rs
layout: document
redirect_from:
- /library/src/ds/list.rs
- /library/src/ds/list.rs.html
title: src/ds/list.rs
---
