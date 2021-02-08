---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/bounded.rs
    title: src/bounded.rs
  - icon: ':heavy_check_mark:'
    path: src/graph/max_flow/edmonds_karp.rs
    title: src/graph/max_flow/edmonds_karp.rs
  - icon: ':heavy_check_mark:'
    path: src/io.rs
    title: src/io.rs
  - icon: ':heavy_check_mark:'
    path: src/num.rs
    title: src/num.rs
  - icon: ':heavy_check_mark:'
    path: src/zo.rs
    title: src/zo.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_6_A
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: test/src/bin/edmonds_karp_test.rs\n"
  code: "// verification-helper: PROBLEM http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_6_A\n\
    \nuse lib::graph::max_flow::edmonds_karp::EdmondsKarp;\nuse lib::io::*;\n\nfn\
    \ main() {\n    let mut io = IO::new();\n    let [n, m]: [usize; 2] = io.scan();\n\
    \    let mut solver = EdmondsKarp::<u32>::new(n);\n    for _ in 0..m {\n     \
    \   solver.add_edge(io.scan(), io.scan(), io.scan());\n    }\n    io.println(solver.solve(0,\
    \ n - 1));\n}\n"
  dependsOn:
  - src/bounded.rs
  - src/graph/max_flow/edmonds_karp.rs
  - src/io.rs
  - src/num.rs
  - src/zo.rs
  isVerificationFile: true
  path: test/src/bin/edmonds_karp_test.rs
  requiredBy: []
  timestamp: '2021-02-08 00:55:24+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: test/src/bin/edmonds_karp_test.rs
layout: document
redirect_from:
- /verify/test/src/bin/edmonds_karp_test.rs
- /verify/test/src/bin/edmonds_karp_test.rs.html
title: test/src/bin/edmonds_karp_test.rs
---