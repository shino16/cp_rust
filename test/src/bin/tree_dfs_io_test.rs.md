---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/alg.rs
    title: src/alg.rs
  - icon: ':heavy_check_mark:'
    path: src/alg/arith.rs
    title: src/alg/arith.rs
  - icon: ':heavy_check_mark:'
    path: src/bit.rs
    title: src/bit.rs
  - icon: ':heavy_check_mark:'
    path: src/ds/fenwick.rs
    title: src/ds/fenwick.rs
  - icon: ':heavy_check_mark:'
    path: src/graph.rs
    title: src/graph.rs
  - icon: ':heavy_check_mark:'
    path: src/graph/tree.rs
    title: src/graph/tree.rs
  - icon: ':heavy_check_mark:'
    path: src/graph/tree/dfs_io.rs
    title: src/graph/tree/dfs_io.rs
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
    PROBLEM: https://yukicoder.me/problems/no/778
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: test/src/bin/tree_dfs_io_test.rs\n"
  code: "// verify-helper: PROBLEM https://yukicoder.me/problems/no/778\n\nuse lib::ds::fenwick::*;\n\
    use lib::graph::tree::dfs_io::*;\nuse lib::io::*;\n\nfn main() {\n\tlet mut io\
    \ = IO::new();\n\tlet n = io.scan();\n\tlet mut graph = vec![Vec::new(); n];\n\
    \tfor v in 1..n {\n\t\tlet p: usize = io.scan();\n\t\tgraph[p].push(v);\n\t\t\
    graph[v].push(p);\n\t}\n\tlet mut fwk = FenwickTree::new(vec![0; n], Addition::new());\n\
    \tlet mut ans = 0;\n\tdfs_io(&graph, 0, |v, _| match v {\n\t\tIn(v) => {\n\t\t\
    \tans += fwk.ask_prefix(v) as u64;\n\t\t\tfwk.add(v, 1_u32);\n\t\t},\n\t\tOut(v)\
    \ => fwk.sub(v, 1),\n\t});\n\tio.println(ans);\n}\n"
  dependsOn:
  - src/alg.rs
  - src/alg/arith.rs
  - src/bit.rs
  - src/ds/fenwick.rs
  - src/graph.rs
  - src/graph/tree.rs
  - src/graph/tree/dfs_io.rs
  - src/io.rs
  - src/num.rs
  - src/zo.rs
  isVerificationFile: true
  path: test/src/bin/tree_dfs_io_test.rs
  requiredBy: []
  timestamp: '2021-01-31 21:57:49+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: test/src/bin/tree_dfs_io_test.rs
layout: document
redirect_from:
- /verify/test/src/bin/tree_dfs_io_test.rs
- /verify/test/src/bin/tree_dfs_io_test.rs.html
title: test/src/bin/tree_dfs_io_test.rs
---
