---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/alg.rs
    title: src/alg.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/segtree_beats_test.rs
    title: test/src/bin/segtree_beats_test.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/ds/segtree/beats.rs\n"
  code: "pub use crate::alg::*;\n\nfn trunc(x: usize) -> usize {\n\tx >> x.trailing_zeros()\n\
    }\n\n#[derive(Clone)]\npub struct SegmentTreeBeats<On: Monoid, Act: Monoid, Apply>\n\
    where\n\tApply: Fn(On::Item, Act::Item) -> Option<On::Item>,\n{\n\tlen: usize,\n\
    \tlog: u32,\n\tdata: Vec<(On::Item, Act::Item)>,\n\ton_alg: On,\n\tact_alg: Act,\n\
    \tapply: Apply,\n}\n\nimpl<On: Monoid, Act: Monoid, Apply: Fn(On::Item, Act::Item)\
    \ -> Option<On::Item>>\n\tSegmentTreeBeats<On, Act, Apply>\n{\n\tpub fn new(len:\
    \ usize, on_alg: On, act_alg: Act, apply: Apply) -> Self {\n\t\tSelf {\n\t\t\t\
    len,\n\t\t\tlog: len.next_power_of_two().trailing_zeros(),\n\t\t\tdata: vec![(on_alg.unit(),\
    \ act_alg.unit()); len * 2],\n\t\t\ton_alg,\n\t\t\tact_alg,\n\t\t\tapply,\n\t\t\
    }\n\t}\n\tpub fn from_slice(slice: &[On::Item], on_alg: On, act_alg: Act, apply:\
    \ Apply) -> Self {\n\t\tlet len = slice.len();\n\t\tlet log = len.next_power_of_two().trailing_zeros();\n\
    \t\tlet iter = slice.iter().map(|&x| (x, act_alg.unit()));\n\t\tlet mut data:\
    \ Vec<_> = iter.clone().chain(iter).collect();\n\t\tfor i in (1..len).rev() {\n\
    \t\t\tdata[i].0 = on_alg.op(data[i << 1].0, data[i << 1 | 1].0);\n\t\t}\n\t\t\
    Self { len, log, data, on_alg, act_alg, apply }\n\t}\n\tpub fn len(&self) -> usize\
    \ {\n\t\tself.len\n\t}\n\tfn apply(&mut self, p: usize, actor: Act::Item) {\n\t\
    \tself.act_alg.op_to(actor, &mut self.data[p].1);\n\t\tself.data[p].0 = if let\
    \ Some(d) = (self.apply)(self.data[p].0, actor) {\n\t\t\td\n\t\t} else {\n\t\t\
    \tself.push(p);\n\t\t\tself.on_alg.op(self.data[p << 1].0, self.data[p << 1 |\
    \ 1].0)\n\t\t};\n\t}\n\tfn push(&mut self, p: usize) {\n\t\tself.apply(p << 1,\
    \ self.data[p].1);\n\t\tself.apply(p << 1 | 1, self.data[p].1);\n\t\tself.data[p].1\
    \ = self.act_alg.unit();\n\t}\n\tfn flush(&mut self, p: usize) {\n\t\tfor s in\
    \ (1..=self.log).rev() {\n\t\t\tself.push(p >> s);\n\t\t}\n\t}\n\tfn build(&mut\
    \ self, mut p: usize) {\n\t\tp >>= 1;\n\t\twhile p != 0 {\n\t\t\tself.data[p].0\
    \ = self.on_alg.op(self.data[p << 1].0, self.data[p << 1 | 1].0);\n\t\t\t// debug_assert_eq!(self.data[p].1,\
    \ self.act_alg.unit());\n\t\t\tp >>= 1;\n\t\t}\n\t}\n\tpub fn ask(&mut self, l:\
    \ usize, r: usize) -> On::Item {\n\t\tself.flush(trunc(l + self.len()));\n\t\t\
    self.flush(trunc(r + self.len()) - 1);\n\t\tlet [mut resl, mut resr] = [self.on_alg.unit();\
    \ 2];\n\t\tlet (mut l, mut r) = (l + self.len(), r + self.len());\n\t\twhile l\
    \ < r {\n\t\t\tif l & 1 != 0 {\n\t\t\t\tresl = self.on_alg.op(resl, self.data[l].0);\n\
    \t\t\t\tl += 1;\n\t\t\t}\n\t\t\tif r & 1 != 0 {\n\t\t\t\tr -= 1;\n\t\t\t\tresr\
    \ = self.on_alg.op(self.data[r].0, resr);\n\t\t\t}\n\t\t\tl >>= 1;\n\t\t\tr >>=\
    \ 1;\n\t\t}\n\t\tself.on_alg.op(resl, resr)\n\t}\n\tpub fn exec<F: FnOnce(&mut\
    \ On::Item)>(&mut self, pos: usize, f: F) {\n\t\tself.flush(pos + self.len());\n\
    \t\tlet p = pos + self.len();\n\t\tf(&mut self.data[p].0);\n\t\tself.build(pos\
    \ + self.len());\n\t}\n\tpub fn act_over(&mut self, l: usize, r: usize, actor:\
    \ Act::Item) {\n\t\tself.flush(trunc(l + self.len()));\n\t\tself.flush(trunc(r\
    \ + self.len()) - 1);\n\t\t{\n\t\t\tlet (mut l, mut r) = (l + self.len(), r +\
    \ self.len());\n\t\t\twhile l < r {\n\t\t\t\tif l & 1 != 0 {\n\t\t\t\t\tself.apply(l,\
    \ actor);\n\t\t\t\t\tl += 1;\n\t\t\t\t}\n\t\t\t\tif r & 1 != 0 {\n\t\t\t\t\tr\
    \ -= 1;\n\t\t\t\t\tself.apply(r, actor);\n\t\t\t\t}\n\t\t\t\tl >>= 1;\n\t\t\t\t\
    r >>= 1;\n\t\t\t}\n\t\t}\n\t\tself.build(trunc(l + self.len()));\n\t\tself.build(trunc(r\
    \ + self.len()) - 1);\n\t}\n}\n"
  dependsOn:
  - src/alg.rs
  isVerificationFile: false
  path: src/ds/segtree/beats.rs
  requiredBy: []
  timestamp: '2021-02-05 04:21:11+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/segtree_beats_test.rs
documentation_of: src/ds/segtree/beats.rs
layout: document
redirect_from:
- /library/src/ds/segtree/beats.rs
- /library/src/ds/segtree/beats.rs.html
title: src/ds/segtree/beats.rs
---
