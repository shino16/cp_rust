---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/ds/list.rs\n"
  code: "use std::iter::FromIterator;\nuse std::rc::Rc;\n\n#[derive(Clone, Ord, PartialOrd,\
    \ Eq, PartialEq, Debug)]\npub struct List<T>(Option<Rc<(T, List<T>)>>);\n\nimpl<T>\
    \ Default for List<T> {\n    fn default() -> Self {\n        Self::new()\n   \
    \ }\n}\n\nimpl<T> List<T> {\n    pub fn new() -> Self {\n        Self(None)\n\
    \    }\n    pub fn push(&mut self, val: T) {\n        unsafe {\n            std::ptr::write(self,\
    \ Self(Some(Rc::new((val, std::ptr::read(self))))));\n        }\n    }\n    pub\
    \ fn cons(self, val: T) -> Self {\n        Self(Some(Rc::new((val, self))))\n\
    \    }\n    pub fn head(&self) -> Option<&T> {\n        self.0.as_deref().map(|(hd,\
    \ _)| hd)\n    }\n    pub fn tail(&self) -> Option<&Self> {\n        self.0.as_deref().map(|(_,\
    \ tl)| tl)\n    }\n    pub fn from_rev_iter<I: IntoIterator<Item = T>>(iter: I)\
    \ -> Self {\n        iter.into_iter().fold(Self::new(), |es, e| es.cons(e))\n\
    \    }\n}\n\npub struct Iter<'a, T>(&'a List<T>);\n\nimpl<'a, T> Iterator for\
    \ Iter<'a, T> {\n    type Item = &'a T;\n    fn next(&mut self) -> Option<Self::Item>\
    \ {\n        match (self.0).0.as_deref() {\n            None => None,\n      \
    \      Some(&(ref hd, ref tl)) => {\n                *self = Self(tl);\n     \
    \           Some(hd)\n            },\n        }\n    }\n}\n\nimpl<T> FromIterator<T>\
    \ for List<T> {\n    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self\
    \ {\n        let mut iter = iter.into_iter();\n        let val = iter.next();\n\
    \        Self(val.map(|val| Rc::new((val, iter.collect()))))\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/ds/list.rs
  requiredBy: []
  timestamp: '2021-02-08 00:55:24+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/ds/list.rs
layout: document
redirect_from:
- /library/src/ds/list.rs
- /library/src/ds/list.rs.html
title: src/ds/list.rs
---
