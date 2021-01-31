---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: src/alg.rs
    title: src/alg.rs
  - icon: ':heavy_check_mark:'
    path: src/alg/action.rs
    title: src/alg/action.rs
  - icon: ':question:'
    path: src/alg/arith.rs
    title: src/alg/arith.rs
  - icon: ':heavy_check_mark:'
    path: src/ds/segtree/lazy.rs
    title: src/ds/segtree/lazy.rs
  - icon: ':question:'
    path: src/io.rs
    title: src/io.rs
  - icon: ':heavy_check_mark:'
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
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/range_affine_range_sum
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: test/src/bin/lazy_segtree_test.rs\n"
  code: "// verify-helper: PROBLEM https://judge.yosupo.jp/problem/range_affine_range_sum\n\
    \nuse lib::alg::arith::*;\nuse lib::ds::segtree::lazy::*;\nuse lib::io::*;\nuse\
    \ lib::mint::*;\n\nfn main() {\n\tlet mut io = IO::new();\n\tlet [n, q]: [usize;\
    \ 2] = io.scan();\n\tlet a = io.scan_iter::<Mint99>(n).map(|a| (a, Mint99::ONE)).collect::<Vec<_>>();\n\
    \tlet mut ds = LazySegmentTree::new_from_slice(\n\t\t&a,\n\t\tMonoidImpl(|| (Mint99::ZERO,\
    \ Mint99::ZERO), |(a, s), (b, t)| (a + b, s + t)),\n\t\tMonoidImpl(|| (Mint99::ONE,\
    \ Mint99::ZERO), |(a, b), (c, d)| (a * c, b * c + d)),\n\t\t|(x, w), (a, b)| (a\
    \ * x + b * w, w),\n\t);\n\tfor _ in 0..q {\n\t\tlet t: u32 = io.scan();\n\t\t\
    if t == 0 {\n\t\t\tds.act_over(io.scan(), io.scan(), io.scan());\n\t\t} else {\n\
    \t\t\tlet ans = ds.ask(io.scan(), io.scan()).0;\n\t\t\tio.println(ans);\n\t\t\
    }\n\t}\n}\n\n// f(x) = ax + b\n// g(x) = cx + d\n// g(f(x)) = acx + bc + d\n"
  dependsOn:
  - src/alg.rs
  - src/alg/action.rs
  - src/alg/arith.rs
  - src/ds/segtree/lazy.rs
  - src/io.rs
  - src/mint.rs
  - src/num.rs
  - src/zo.rs
  isVerificationFile: true
  path: test/src/bin/lazy_segtree_test.rs
  requiredBy: []
  timestamp: '2021-01-31 20:22:45+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: test/src/bin/lazy_segtree_test.rs
layout: document
redirect_from:
- /verify/test/src/bin/lazy_segtree_test.rs
- /verify/test/src/bin/lazy_segtree_test.rs.html
title: test/src/bin/lazy_segtree_test.rs
---
