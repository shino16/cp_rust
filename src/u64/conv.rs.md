---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: src/bit.rs
    title: src/bit.rs
  - icon: ':question:'
    path: src/cast.rs
    title: src/cast.rs
  - icon: ':heavy_check_mark:'
    path: src/conv.rs
    title: src/conv.rs
  - icon: ':heavy_check_mark:'
    path: src/ds/uvec.rs
    title: src/ds/uvec.rs
  - icon: ':question:'
    path: src/fp.rs
    title: src/fp.rs
  - icon: ':heavy_check_mark:'
    path: src/fp/conv.rs
    title: src/fp/conv.rs
  - icon: ':question:'
    path: src/int.rs
    title: src/int.rs
  - icon: ':question:'
    path: src/io.rs
    title: src/io.rs
  - icon: ':question:'
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
    RuntimeError: bundler is not specified: src/u64/conv.rs\n"
  code: "pub use crate::conv::*;\nuse crate::fp::conv::*;\n\nimpl Conv for u64 {\n\
    \tfn conv_in_place(lhs: &mut Vec<Self>, rhs: &mut Vec<Self>) {\n\t\tlet r12 =\
    \ FpC::from(FpB::P).inv();\n\t\tlet r13 = FpD::from(FpB::P).inv();\n\t\tlet r23\
    \ = FpD::from(FpC::P).inv();\n\t\tfn run<M: Mod>(lhs: &mut Vec<u64>, rhs: &mut\
    \ Vec<u64>) -> Vec<Fp<M>>\n\t\twhere\n\t\t\tFp<M>: Conv,\n\t\t{\n\t\t\tlet lhs\
    \ = lhs.iter().map(|&e| Fp::from(e)).collect();\n\t\t\tlet rhs = rhs.iter().map(|&e|\
    \ Fp::from(e)).collect();\n\t\t\tConv::conv(lhs, rhs)\n\t\t}\n\t\tlet v1: Vec<FpB>\
    \ = run(lhs, rhs);\n\t\tlet v2: Vec<FpC> = run(lhs, rhs);\n\t\tlet v3: Vec<FpD>\
    \ = run(lhs, rhs);\n\t\tlhs.resize(v1.len(), Default::default());\n\t\tfor (((e0,\
    \ e1), e2), e3) in lhs.iter_mut().zip(v1).zip(v2).zip(v3) {\n\t\t\tlet x1 = e1;\n\
    \t\t\tlet x2 = (e2 - x1.value()) * r12;\n\t\t\tlet x3 = ((e3 - x1.value()) * r13\
    \ - x2.value()) * r23;\n\t\t\tlet mut x = x1.value() as u64;\n\t\t\tx += x2.value()\
    \ as u64 * FpB::P as u64;\n\t\t\tx += x3.value() as u64 * FpB::P as u64 * FpC::P\
    \ as u64;\n\t\t\t*e0 = x;\n\t\t}\n\t}\n}\n"
  dependsOn:
  - src/bit.rs
  - src/cast.rs
  - src/conv.rs
  - src/ds/uvec.rs
  - src/fp.rs
  - src/fp/conv.rs
  - src/int.rs
  - src/io.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/u64/conv.rs
  requiredBy: []
  timestamp: '2021-01-07 16:16:30+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/u64/conv.rs
layout: document
redirect_from:
- /library/src/u64/conv.rs
- /library/src/u64/conv.rs.html
title: src/u64/conv.rs
---
