---
data:
  _extendedDependsOn:
  - icon: ':x:'
    path: src/as_int.rs
    title: src/as_int.rs
  - icon: ':x:'
    path: src/bit.rs
    title: src/bit.rs
  - icon: ':x:'
    path: src/dfa.rs
    title: src/dfa.rs
  - icon: ':x:'
    path: src/io.rs
    title: src/io.rs
  - icon: ':x:'
    path: src/modint.rs
    title: src/modint.rs
  - icon: ':x:'
    path: src/num.rs
    title: src/num.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _pathExtension: rs
  _verificationStatusIcon: ':x:'
  attributes:
    PROBLEM: http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=0570
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "// verify-helper: PROBLEM http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=0570\n\
    \nuse lib::dfa::*;\nuse lib::io::*;\nuse lib::modint::*;\n\n#[derive(Ord, PartialOrd,\
    \ Eq, PartialEq, Copy, Clone, Hash)]\nenum ZigZagState {\n    Initial,\n    First(u8),\n\
    \    Increasing(u8),\n    Decreasing(u8),\n}\n\nstruct ZigZag;\n\nimpl Dfa for\
    \ ZigZag {\n    type Alphabet = u8;\n    type State = Option<ZigZagState>;\n \
    \   fn init(&self) -> Self::State {\n        Some(ZigZagState::Initial)\n    }\n\
    \    fn next(&self, s: Self::State, a: Self::Alphabet, _: usize) -> Self::State\
    \ {\n        use ZigZagState::*;\n        if let Some(s) = s {\n            match\
    \ s {\n                Initial if a == b'0' => Some(Initial),\n              \
    \  Initial => Some(First(a)),\n                First(d) if d < a => Some(Increasing(a)),\n\
    \                First(d) if d > a => Some(Decreasing(a)),\n                Increasing(d)\
    \ if d > a => Some(Decreasing(a)),\n                Decreasing(d) if d < a =>\
    \ Some(Increasing(a)),\n                _ => None,\n            }\n        } else\
    \ {\n            None\n        }\n    }\n    fn accept(&self, s: Self::State)\
    \ -> bool {\n        s.is_some()\n    }\n    fn unsuccessful(&self, s: Self::State)\
    \ -> bool {\n        s.is_none()\n    }\n}\n\n#[derive(Default, Clone, Copy, PartialEq,\
    \ Eq)]\nstruct Modx;\n\nimpl Mod for Modx {\n    const P: u32 = 10000;\n    const\
    \ PHI: u32 = 4000;\n}\n\nfn main() {\n    let mut io = IO::new();\n    let [a0,\
    \ b]: [&[u8]; 2] = io.scan();\n    let m = io.scan();\n    let mut a = vec![b'0';\
    \ b.len()];\n    a[b.len() - a0.len()..].copy_from_slice(&a0);\n\n    let dfa\
    \ = And(ZigZag, And(MultipleOf(m), And(Leq(&b), Not(Lt(&a)))));\n    let alphabet\
    \ = \"0123456789\".as_bytes();\n    let ans: Modint<Modx> = dfa.count(a.len(),\
    \ alphabet);\n    println!(\"{}\", ans);\n}\n"
  dependsOn:
  - src/as_int.rs
  - src/bit.rs
  - src/dfa.rs
  - src/io.rs
  - src/modint.rs
  - src/num.rs
  isVerificationFile: true
  path: test/src/bin/dfa_test.rs
  requiredBy: []
  timestamp: '2020-11-17 21:23:08+09:00'
  verificationStatus: TEST_WRONG_ANSWER
  verifiedWith: []
documentation_of: test/src/bin/dfa_test.rs
layout: document
redirect_from:
- /verify/test/src/bin/dfa_test.rs
- /verify/test/src/bin/dfa_test.rs.html
title: test/src/bin/dfa_test.rs
---
