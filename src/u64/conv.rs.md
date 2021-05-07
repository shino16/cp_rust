---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/conv.rs
    title: src/conv.rs
  - icon: ':heavy_check_mark:'
    path: src/ds.rs
    title: src/ds.rs
  - icon: ':heavy_check_mark:'
    path: src/ds/uvec.rs
    title: src/ds/uvec.rs
  - icon: ':heavy_check_mark:'
    path: src/gf.rs
    title: src/gf.rs
  - icon: ':heavy_check_mark:'
    path: src/gf/conv.rs
    title: src/gf/conv.rs
  - icon: ':heavy_check_mark:'
    path: src/zo.rs
    title: src/zo.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.5/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.5/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/u64/conv.rs\n"
  code: "pub use crate::conv::*;\nuse crate::gf::conv::*;\n\nimpl Conv for u64 {\n\
    \    fn conv_in_place(lhs: &mut Vec<Self>, rhs: &mut Vec<Self>) {\n        let\
    \ r12 = GfC::from(GfB::P).inv();\n        let r13 = GfD::from(GfB::P).inv();\n\
    \        let r23 = GfD::from(GfC::P).inv();\n        fn run<M: Mod>(lhs: &mut\
    \ Vec<u64>, rhs: &mut Vec<u64>) -> Vec<Gf<M>>\n        where\n            Gf<M>:\
    \ Conv,\n        {\n            let lhs = lhs.iter().map(|&e| Gf::from(e)).collect();\n\
    \            let rhs = rhs.iter().map(|&e| Gf::from(e)).collect();\n         \
    \   Conv::conv(lhs, rhs)\n        }\n        let v1: Vec<GfB> = run(lhs, rhs);\n\
    \        let v2: Vec<GfC> = run(lhs, rhs);\n        let v3: Vec<GfD> = run(lhs,\
    \ rhs);\n        lhs.resize_with(v1.len(), Default::default);\n        for (((e0,\
    \ e1), e2), e3) in lhs.iter_mut().zip(v1).zip(v2).zip(v3) {\n            let x1\
    \ = e1;\n            let x2 = (e2 - x1.value()) * r12;\n            let x3 = ((e3\
    \ - x1.value()) * r13 - x2.value()) * r23;\n            let mut x = x1.value()\
    \ as u64;\n            x += x2.value() as u64 * GfB::P as u64;\n            x\
    \ += x3.value() as u64 * GfB::P as u64 * GfC::P as u64;\n            *e0 = x;\n\
    \        }\n    }\n}\n"
  dependsOn:
  - src/conv.rs
  - src/ds.rs
  - src/ds/uvec.rs
  - src/gf.rs
  - src/gf/conv.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/u64/conv.rs
  requiredBy: []
  timestamp: '2021-04-11 12:36:47+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/u64/conv.rs
layout: document
redirect_from:
- /library/src/u64/conv.rs
- /library/src/u64/conv.rs.html
title: src/u64/conv.rs
---
