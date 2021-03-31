---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: src/float.rs
    title: src/float.rs
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
    RuntimeError: bundler is not specified: src/float/convex.rs\n"
  code: "use super::*;\n\n/// return (min f, argmin f)\npub fn convex_min<T: PartialOrd>(\n\
    \    mut l: Float,\n    mut r: Float,\n    e: Float,\n    mut f: impl FnMut(Float)\
    \ -> T,\n) -> (T, Float) {\n    const PHI: Float = 1.618_033_988_749_895;\n  \
    \  let k = ((r - l) / e).log(PHI) as u32 + 2;\n\n    let mut ml = (PHI * l + r)\
    \ / (1.0 + PHI);\n    let mut mr = (l + PHI * r) / (1.0 + PHI);\n    let mut yml\
    \ = f(ml);\n    let mut ymr = f(mr);\n\n    for _ in 0..k {\n        if yml <\
    \ ymr {\n            l = ml;\n            ml = mr;\n            yml = ymr;\n \
    \           mr = (ml + PHI * r) / (1.0 + PHI);\n            ymr = f(mr);\n   \
    \     } else {\n            r = mr;\n            mr = ml;\n            ymr = yml;\n\
    \            mr = (PHI * l + mr) / (1.0 + PHI);\n            yml = f(ml);\n  \
    \      }\n    }\n    (yml, ml)\n}\n"
  dependsOn:
  - src/float.rs
  isVerificationFile: false
  path: src/float/convex.rs
  requiredBy: []
  timestamp: '2021-03-31 15:51:17+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/float/convex.rs
layout: document
redirect_from:
- /library/src/float/convex.rs
- /library/src/float/convex.rs.html
title: src/float/convex.rs
---
