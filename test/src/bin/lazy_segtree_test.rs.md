---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: src/alg.rs
    title: src/alg.rs
  - icon: ':question:'
    path: src/alg/arith.rs
    title: src/alg/arith.rs
  - icon: ':x:'
    path: src/ds/segtree/lazy.rs
    title: src/ds/segtree/lazy.rs
  - icon: ':question:'
    path: src/io.rs
    title: src/io.rs
  - icon: ':question:'
    path: src/mint.rs
    title: src/mint.rs
  - icon: ':question:'
    path: src/num.rs
    title: src/num.rs
  - icon: ':question:'
    path: src/zo.rs
    title: src/zo.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':x:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/range_affine_range_sum
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: test/src/bin/lazy_segtree_test.rs\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/range_affine_range_sum\n\
    \nuse lib::alg::arith::*;\nuse lib::ds::segtree::lazy::*;\nuse lib::io::*;\nuse\
    \ lib::mint::*;\n\nfn main() {\n    let mut io = IO::new();\n    let [n, q]: [usize;\
    \ 2] = io.scan();\n    let a = io.scan_iter::<Mint99>(n).map(|a| (a, Mint99::ONE)).collect::<Vec<_>>();\n\
    \    let mut ds = LazySegmentTree::from_slice(\n        &a,\n        MonoidImpl(||\
    \ (Mint99::ZERO, Mint99::ZERO), |(a, s), (b, t)| (a + b, s + t)),\n        MonoidImpl(||\
    \ (Mint99::ONE, Mint99::ZERO), |(a, b), (c, d)| (a * c, b * c + d)),\n       \
    \ |(x, w), (a, b)| (a * x + b * w, w),\n    );\n    for _ in 0..q {\n        let\
    \ t: u32 = io.scan();\n        if t == 0 {\n            ds.act_over(io.scan(),\
    \ io.scan(), io.scan());\n        } else {\n            let ans = ds.ask(io.scan(),\
    \ io.scan()).0;\n            io.println(ans);\n        }\n    }\n}\n\n// f(x)\
    \ = ax + b\n// g(x) = cx + d\n// g(f(x)) = acx + bc + d\n"
  dependsOn:
  - src/alg.rs
  - src/alg/arith.rs
  - src/ds/segtree/lazy.rs
  - src/io.rs
  - src/mint.rs
  - src/num.rs
  - src/zo.rs
  isVerificationFile: true
  path: test/src/bin/lazy_segtree_test.rs
  requiredBy: []
  timestamp: '2021-02-08 00:55:36+09:00'
  verificationStatus: TEST_WRONG_ANSWER
  verifiedWith: []
documentation_of: test/src/bin/lazy_segtree_test.rs
layout: document
redirect_from:
- /verify/test/src/bin/lazy_segtree_test.rs
- /verify/test/src/bin/lazy_segtree_test.rs.html
title: test/src/bin/lazy_segtree_test.rs
---
