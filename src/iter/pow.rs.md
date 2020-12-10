---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/cargo_test.rs
    title: test/src/bin/cargo_test.rs
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "/// pow_iter(0..10, 8): 700 ms (AtC) / 900 ms (CF)\npub fn pow_iter<I: Iterator\
    \ + Clone>(iter: I, k: usize) -> IterPow<I> {\n\tIterPow {\n\t\titers: vec![iter.clone();\
    \ k],\n\t\titers0: vec![iter; k],\n\t\tstate: Vec::with_capacity(k),\n\t}\n}\n\
    \npub struct IterPow<I: Iterator + Clone> {\n\titers: Vec<I>,\n\titers0: Vec<I>,\n\
    \tstate: Vec<I::Item>,\n}\n\nimpl<'a, I: Iterator + Clone> IterPow<I>\nwhere\n\
    \tI::Item: Clone,\n{\n\tpub fn next(&mut self) -> Option<&Vec<I::Item>> {\n\t\t\
    if self.state.len() == 0 {\n\t\t\tfor iter in self.iters.iter_mut() {\n\t\t\t\t\
    self.state.push(iter.next()?);\n\t\t\t}\n\t\t\treturn Some(&self.state);\n\t\t\
    }\n\t\tfor ((iter, iter0), state) in\n\t\t\tself.iters.iter_mut().zip(self.iters0.iter()).zip(self.state.iter_mut())\n\
    \t\t{\n\t\t\tif let Some(e) = iter.next() {\n\t\t\t\t*state = e;\n\t\t\t\treturn\
    \ Some(&self.state);\n\t\t\t}\n\t\t\t*iter = iter0.clone();\n\t\t}\n\t\tNone\n\
    \t}\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/iter/pow.rs
  requiredBy: []
  timestamp: '2020-12-10 17:35:58+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/cargo_test.rs
documentation_of: src/iter/pow.rs
layout: document
redirect_from:
- /library/src/iter/pow.rs
- /library/src/iter/pow.rs.html
title: src/iter/pow.rs
---
