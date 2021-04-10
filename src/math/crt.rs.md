---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: src/math/gcd/ext.rs
    title: src/math/gcd/ext.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.4/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.4/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/math/crt.rs\n"
  code: "use std::u64;\n\nuse super::gcd::ext::*;\n\npub fn crt(rm: &[(u64, u64)])\
    \ -> (u64, u64) {\n    let (mut r, mut m) = (0, 1);\n    for &(ri, mi) in rm {\n\
    \        let (mut ri, mut mi) = ((ri % mi) as i64, mi as i64);\n        if m <\
    \ mi {\n            std::mem::swap(&mut m, &mut mi);\n            std::mem::swap(&mut\
    \ r, &mut ri);\n        }\n        if m % mi == 0 {\n            if r % mi !=\
    \ ri {\n                return (0, 0);\n            }\n        } else {\n    \
    \        let (g, im) = extgcd(m as u64, mi as u64);\n            let (g, im) =\
    \ (g as i64, im as i64);\n            if (ri - r) % g != 0 {\n               \
    \ return (0, 0);\n            }\n            let u = mi / g;\n            r +=\
    \ (ri - r) / g % u * im % u * m;\n            m *= u;\n            if r < 0 {\n\
    \                r += m;\n            }\n        }\n    }\n    (r as u64, m as\
    \ u64)\n}\n"
  dependsOn:
  - src/math/gcd/ext.rs
  isVerificationFile: false
  path: src/math/crt.rs
  requiredBy: []
  timestamp: '2021-02-28 10:03:54+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/math/crt.rs
layout: document
redirect_from:
- /library/src/math/crt.rs
- /library/src/math/crt.rs.html
title: src/math/crt.rs
---
