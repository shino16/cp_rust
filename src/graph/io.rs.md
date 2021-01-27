---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/io.rs
    title: src/io.rs
  _extendedRequiredBy:
  - icon: ':warning:'
    path: src/io/graph.rs
    title: src/io/graph.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/graph/io.rs\n"
  code: "pub use crate::io::*;\n\nimpl IO {\n\tpub fn scan_tree(&mut self) -> (Vec<Vec<usize>>,\
    \ usize) {\n\t\tlet n = self.scan();\n\t\tlet mut graph = vec![Vec::new(); n];\n\
    \t\tfor _ in 0..n - 1 {\n\t\t\tlet (Usize1(u), Usize1(v)) = self.scan();\n\t\t\
    \tgraph[u].push(v);\n\t\t\tgraph[v].push(u);\n\t\t}\n\t\t(graph, n)\n\t}\n\tpub\
    \ fn scan_graph(&mut self) -> (Vec<Vec<usize>>, usize, usize) {\n\t\tlet (n, m)\
    \ = self.scan();\n\t\tlet mut graph = vec![Vec::new(); n];\n\t\tfor _ in 0..m\
    \ {\n\t\t\tlet (Usize1(u), Usize1(v)) = self.scan();\n\t\t\tgraph[u].push(v);\n\
    \t\t\tgraph[v].push(u);\n\t\t}\n\t\t(graph, n, m)\n\t}\n\tpub fn scan_ditree(&mut\
    \ self) -> (Vec<Vec<usize>>, usize) {\n\t\tlet n = self.scan();\n\t\tlet mut graph\
    \ = vec![Vec::new(); n];\n\t\tfor _ in 0..n - 1 {\n\t\t\tlet (Usize1(u), Usize1(v))\
    \ = self.scan();\n\t\t\tgraph[u].push(v);\n\t\t}\n\t\t(graph, n)\n\t}\n\tpub fn\
    \ scan_digraph(&mut self) -> (Vec<Vec<usize>>, usize, usize) {\n\t\tlet (n, m)\
    \ = self.scan();\n\t\tlet mut graph = vec![Vec::new(); n];\n\t\tfor _ in 0..m\
    \ {\n\t\t\tlet (Usize1(u), Usize1(v)) = self.scan();\n\t\t\tgraph[u].push(v);\n\
    \t\t}\n\t\t(graph, n, m)\n\t}\n\tpub fn scan_wtree<W: Scan + Copy>(&mut self)\
    \ -> (Vec<Vec<(usize, W)>>, usize) {\n\t\tlet n = self.scan();\n\t\tlet mut graph\
    \ = vec![Vec::new(); n];\n\t\tfor _ in 0..n - 1 {\n\t\t\tlet (Usize1(u), Usize1(v))\
    \ = self.scan();\n\t\t\tlet w: W = self.scan();\n\t\t\tgraph[u].push((v, w));\n\
    \t\t\tgraph[v].push((u, w));\n\t\t}\n\t\t(graph, n)\n\t}\n\tpub fn scan_wgraph<W:\
    \ Scan + Copy>(&mut self) -> (Vec<Vec<(usize, W)>>, usize, usize) {\n\t\tlet (n,\
    \ m) = self.scan();\n\t\tlet mut graph = vec![Vec::new(); n];\n\t\tfor _ in 0..m\
    \ {\n\t\t\tlet (Usize1(u), Usize1(v)) = self.scan();\n\t\t\tlet w: W = self.scan();\n\
    \t\t\tgraph[u].push((v, w));\n\t\t\tgraph[v].push((u, w));\n\t\t}\n\t\t(graph,\
    \ n, m)\n\t}\n\tpub fn scan_wditree<W: Scan + Copy>(&mut self) -> (Vec<Vec<(usize,\
    \ W)>>, usize) {\n\t\tlet n = self.scan();\n\t\tlet mut graph = vec![Vec::new();\
    \ n];\n\t\tfor _ in 0..n - 1 {\n\t\t\tlet (Usize1(u), Usize1(v)) = self.scan();\n\
    \t\t\tlet w: W = self.scan();\n\t\t\tgraph[u].push((v, w));\n\t\t}\n\t\t(graph,\
    \ n)\n\t}\n\tpub fn scan_wdigraph<W: Scan + Copy>(&mut self) -> (Vec<Vec<(usize,\
    \ W)>>, usize, usize) {\n\t\tlet (n, m) = self.scan();\n\t\tlet mut graph = vec![Vec::new();\
    \ n];\n\t\tfor _ in 0..m {\n\t\t\tlet (Usize1(u), Usize1(v)) = self.scan();\n\t\
    \t\tlet w: W = self.scan();\n\t\t\tgraph[u].push((v, w));\n\t\t}\n\t\t(graph,\
    \ n, m)\n\t}\n}\n"
  dependsOn:
  - src/io.rs
  isVerificationFile: false
  path: src/graph/io.rs
  requiredBy:
  - src/io/graph.rs
  timestamp: '2021-01-27 17:46:37+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/graph/io.rs
layout: document
redirect_from:
- /library/src/graph/io.rs
- /library/src/graph/io.rs.html
title: src/graph/io.rs
---
