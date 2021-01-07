---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "pub trait ProdIterator: Iterator {\n\tfn prod<J: IntoIterator>(mut self,\
    \ other: J) -> Product<Self, J::IntoIter>\n\twhere\n\t\tSelf: Sized,\n\t\tSelf::Item:\
    \ Clone,\n\t\tJ::IntoIter: Clone,\n\t{\n\t\tlet other = other.into_iter();\n\t\
    \tProduct {\n\t\t\tae: self.next(),\n\t\t\ta: self,\n\t\t\tb: other.clone(),\n\
    \t\t\tb0: other,\n\t\t}\n\t}\n}\n\nimpl<I: Iterator> ProdIterator for I {}\n\n\
    pub struct Product<I: Iterator, J> {\n\ta: I,\n\tae: Option<I::Item>,\n\tb: J,\n\
    \tb0: J,\n}\n\nimpl<I: Iterator, J: Iterator + Clone> Iterator for Product<I,\
    \ J>\nwhere\n\tI::Item: Clone,\n{\n\ttype Item = (I::Item, J::Item);\n\tfn next(&mut\
    \ self) -> Option<Self::Item> {\n\t\tlet be = match self.b.next() {\n\t\t\tNone\
    \ => {\n\t\t\t\tself.b = self.b0.clone();\n\t\t\t\tmatch self.b.next() {\n\t\t\
    \t\t\tNone => return None,\n\t\t\t\t\tSome(e) => {\n\t\t\t\t\t\tself.ae = self.a.next();\n\
    \t\t\t\t\t\te\n\t\t\t\t\t},\n\t\t\t\t}\n\t\t\t},\n\t\t\tSome(e) => e,\n\t\t};\n\
    \t\tself.ae.as_ref().map(|ae| (ae.clone(), be))\n\t}\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/iter/prod.rs
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/iter/prod.rs
layout: document
redirect_from:
- /library/src/iter/prod.rs
- /library/src/iter/prod.rs.html
title: src/iter/prod.rs
---
