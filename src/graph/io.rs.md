---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "pub use crate::io::*;\n\nimpl IO {\n\tpub fn scan_tree(&mut self) -> (usize,\
    \ Vec<Vec<usize>>) {\n\t\tlet n = self.scan();\n\t\tlet mut graph = vec![Vec::new();\
    \ n];\n\t\tfor _ in 0..n - 1 {\n\t\t\tlet [u, v]: [usize; 2] = self.scan();\n\t\
    \t\tgraph[u].push(v);\n\t\t\tgraph[v].push(u);\n\t\t}\n\t\t(n, graph)\n\t}\n\t\
    pub fn scan_graph(&mut self) -> (usize, usize, Vec<Vec<usize>>) {\n\t\tlet (n,\
    \ m) = self.scan();\n\t\tlet mut graph = vec![Vec::new(); n];\n\t\tfor _ in 0..m\
    \ {\n\t\t\tlet [u, v]: [usize; 2] = self.scan();\n\t\t\tgraph[u].push(v);\n\t\t\
    \tgraph[v].push(u);\n\t\t}\n\t\t(n, m, graph)\n\t}\n\tpub fn scan_ditree(&mut\
    \ self) -> (usize, Vec<Vec<usize>>) {\n\t\tlet n = self.scan();\n\t\tlet mut graph\
    \ = vec![Vec::new(); n];\n\t\tfor _ in 0..n - 1 {\n\t\t\tlet [u, v]: [usize; 2]\
    \ = self.scan();\n\t\t\tgraph[u].push(v);\n\t\t}\n\t\t(n, graph)\n\t}\n\tpub fn\
    \ scan_digraph(&mut self) -> (usize, usize, Vec<Vec<usize>>) {\n\t\tlet (n, m)\
    \ = self.scan();\n\t\tlet mut graph = vec![Vec::new(); n];\n\t\tfor _ in 0..m\
    \ {\n\t\t\tlet [u, v]: [usize; 2] = self.scan();\n\t\t\tgraph[u].push(v);\n\t\t\
    }\n\t\t(n, m, graph)\n\t}\n\tpub fn scan_wtree<W: Scan + Copy>(&mut self) -> (usize,\
    \ Vec<Vec<(usize, W)>>) {\n\t\tlet n = self.scan();\n\t\tlet mut graph = vec![Vec::new();\
    \ n];\n\t\tfor _ in 0..n - 1 {\n\t\t\tlet [u, v]: [usize; 2] = self.scan();\n\t\
    \t\tlet w: W = self.scan();\n\t\t\tgraph[u].push((v, w));\n\t\t\tgraph[v].push((u,\
    \ w));\n\t\t}\n\t\t(n, graph)\n\t}\n\tpub fn scan_wgraph<W: Scan + Copy>(&mut\
    \ self) -> (usize, usize, Vec<Vec<(usize, W)>>) {\n\t\tlet (n, m) = self.scan();\n\
    \t\tlet mut graph = vec![Vec::new(); n];\n\t\tfor _ in 0..m {\n\t\t\tlet [u, v]:\
    \ [usize; 2] = self.scan();\n\t\t\tlet w: W = self.scan();\n\t\t\tgraph[u].push((v,\
    \ w));\n\t\t\tgraph[v].push((u, w));\n\t\t}\n\t\t(n, m, graph)\n\t}\n\tpub fn\
    \ scan_wditree<W: Scan + Copy>(&mut self) -> (usize, Vec<Vec<(usize, W)>>) {\n\
    \t\tlet n = self.scan();\n\t\tlet mut graph = vec![Vec::new(); n];\n\t\tfor _\
    \ in 0..n - 1 {\n\t\t\tlet [u, v]: [usize; 2] = self.scan();\n\t\t\tlet w: W =\
    \ self.scan();\n\t\t\tgraph[u].push((v, w));\n\t\t}\n\t\t(n, graph)\n\t}\n\tpub\
    \ fn scan_wdigraph<W: Scan + Copy>(&mut self) -> (usize, usize, Vec<Vec<(usize,\
    \ W)>>) {\n\t\tlet (n, m) = self.scan();\n\t\tlet mut graph = vec![Vec::new();\
    \ n];\n\t\tfor _ in 0..m {\n\t\t\tlet [u, v]: [usize; 2] = self.scan();\n\t\t\t\
    let w: W = self.scan();\n\t\t\tgraph[u].push((v, w));\n\t\t}\n\t\t(n, m, graph)\n\
    \t}\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/graph/io.rs
  requiredBy: []
  timestamp: '2020-12-21 16:30:24+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/graph/io.rs
layout: document
redirect_from:
- /library/src/graph/io.rs
- /library/src/graph/io.rs.html
title: src/graph/io.rs
---
