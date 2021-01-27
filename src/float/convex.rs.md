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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/float/convex.rs\n"
  code: "use super::*;\n\n/// return (argmin f, min f)\npub fn convex_min<F: FnMut(Float)\
    \ -> Float>(\n\tmut l: Float,\n\tmut r: Float,\n\te: Float,\n\tmut f: F,\n) ->\
    \ (Float, Float) {\n\tconst PHI: Float = 1.6180339887498948482;\n\tlet k = ((r\
    \ - l) / e).log(PHI) as u32 + 2;\n\n\tlet mut ml = (PHI * l + r) / (1.0 + PHI);\n\
    \tlet mut mr = (l + PHI * r) / (1.0 + PHI);\n\tlet mut yml = f(ml);\n\tlet mut\
    \ ymr = f(mr);\n\n\tfor _ in 0..k {\n\t\tif yml < ymr {\n\t\t\tl = ml;\n\t\t\t\
    ml = mr;\n\t\t\tyml = ymr;\n\t\t\tmr = (ml + PHI * r) / (1.0 + PHI);\n\t\t\tymr\
    \ = f(mr);\n\t\t} else {\n\t\t\tr = mr;\n\t\t\tmr = ml;\n\t\t\tymr = yml;\n\t\t\
    \tmr = (PHI * l + mr) / (1.0 + PHI);\n\t\t\tyml = f(ml);\n\t\t}\n\t}\n\t(ml, yml)\n\
    }\n"
  dependsOn:
  - src/float.rs
  isVerificationFile: false
  path: src/float/convex.rs
  requiredBy: []
  timestamp: '2020-12-21 16:41:24+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/float/convex.rs
layout: document
redirect_from:
- /library/src/float/convex.rs
- /library/src/float/convex.rs.html
title: src/float/convex.rs
---
