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
  code: "use crate::alg::*;\nuse crate::bit::*;\n\n#[derive(Clone)]\npub struct SparseTable<A:\
    \ Alg> {\n\tdata: Vec<Vec<A::Item>>,\n\talg: A,\n}\n\n/// A: Band (x * x == x)\n\
    impl<A: Monoid> SparseTable<A> {\n\tpub fn new(data: Vec<A::Item>, alg: A) ->\
    \ Self {\n\t\tlet len = data.len();\n\t\tlet height = len.ilog2() as usize;\n\t\
    \tlet mut data = vec![data];\n\t\tfor s in 1..=height {\n\t\t\tlet w = 1 << (s\
    \ - 1);\n\t\t\tlet mut new_row = Vec::with_capacity(data[s - 1].len() - w);\n\t\
    \t\tfor i in 0..data[s - 1].len() - w {\n\t\t\t\tnew_row.push(alg.op(data[s -\
    \ 1][i].clone(), data[s - 1][i + w].clone()));\n\t\t\t}\n\t\t\tdata.push(new_row);\n\
    \t\t}\n\t\tSelf { data, alg }\n\t}\n\tpub fn ask(&self, l: usize, r: usize) ->\
    \ A::Item {\n\t\tif l == r {\n\t\t\tself.alg.unit()\n\t\t} else {\n\t\t\tlet s\
    \ = (r - l).ilog2() as usize;\n\t\t\tlet w = 1 << s;\n\t\t\tself.alg.op(self.data[s][l].clone(),\
    \ self.data[s][r - w].clone())\n\t\t}\n\t}\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/ds/sparsetable.rs
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/ds/sparsetable.rs
layout: document
redirect_from:
- /library/src/ds/sparsetable.rs
- /library/src/ds/sparsetable.rs.html
title: src/ds/sparsetable.rs
---
