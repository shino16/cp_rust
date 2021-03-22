---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: src/tests.rs
    title: src/tests.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/cargo_test.rs
    title: test/src/bin/cargo_test.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/iter/prod.rs\n"
  code: "pub trait ProdIterator: Iterator {\n    fn prod<J: IntoIterator>(mut self,\
    \ other: J) -> Product<Self, J::IntoIter>\n    where\n        Self: Sized,\n \
    \       Self::Item: Clone,\n        J::IntoIter: Clone,\n    {\n        let other\
    \ = other.into_iter();\n        Product {\n            ae: self.next(),\n    \
    \        a: self,\n            b: other.clone(),\n            b0: other,\n   \
    \     }\n    }\n}\n\nimpl<I: Iterator> ProdIterator for I {}\n\npub struct Product<I:\
    \ Iterator, J> {\n    a: I,\n    ae: Option<I::Item>,\n    b: J,\n    b0: J,\n\
    }\n\nimpl<I: Iterator, J: Iterator + Clone> Iterator for Product<I, J>\nwhere\n\
    \    I::Item: Clone,\n{\n    type Item = (I::Item, J::Item);\n    fn next(&mut\
    \ self) -> Option<Self::Item> {\n        let be = match self.b.next() {\n    \
    \        None => {\n                self.b = self.b0.clone();\n              \
    \  match self.b.next() {\n                    None => return None,\n         \
    \           Some(e) => {\n                        self.ae = self.a.next();\n \
    \                       e\n                    },\n                }\n       \
    \     },\n            Some(e) => e,\n        };\n        self.ae.as_ref().map(|ae|\
    \ (ae.clone(), be))\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/iter/prod.rs
  requiredBy:
  - src/tests.rs
  timestamp: '2021-02-08 00:55:24+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/cargo_test.rs
documentation_of: src/iter/prod.rs
layout: document
redirect_from:
- /library/src/iter/prod.rs
- /library/src/iter/prod.rs.html
title: src/iter/prod.rs
---
