---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/alg.rs
    title: src/alg.rs
  - icon: ':heavy_check_mark:'
    path: src/ds/segtree.rs
    title: src/ds/segtree.rs
  - icon: ':question:'
    path: src/io.rs
    title: src/io.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_A
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: test/src/bin/segtree_test.rs\n"
  code: "// verification-helper: PROBLEM http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_A\n\
    \nuse lib::ds::segtree::*;\nuse lib::io::*;\n\nfn main() {\n    let mut io = IO::new();\n\
    \    let (n, q): (usize, usize) = io.scan();\n\n    let inf = (1_u32 << 31) -\
    \ 1;\n\n    let mut st = SegmentTree::new(n, MonoidImpl(|| inf, |a, b| a.min(b)));\n\
    \n    for _ in 0..q {\n        let [c, x, y]: [usize; 3] = io.scan();\n      \
    \  if c == 0 {\n            st.with(x, |v| *v = y as u32);\n        } else {\n\
    \            io.println(st.ask(x, y + 1));\n        }\n    }\n}\n"
  dependsOn:
  - src/alg.rs
  - src/ds/segtree.rs
  - src/io.rs
  isVerificationFile: true
  path: test/src/bin/segtree_test.rs
  requiredBy: []
  timestamp: '2021-02-22 02:21:06+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: test/src/bin/segtree_test.rs
layout: document
redirect_from:
- /verify/test/src/bin/segtree_test.rs
- /verify/test/src/bin/segtree_test.rs.html
title: test/src/bin/segtree_test.rs
---
