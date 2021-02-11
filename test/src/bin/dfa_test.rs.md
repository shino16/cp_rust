---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/bits.rs
    title: src/bits.rs
  - icon: ':heavy_check_mark:'
    path: src/bounded.rs
    title: src/bounded.rs
  - icon: ':heavy_check_mark:'
    path: src/cast.rs
    title: src/cast.rs
  - icon: ':heavy_check_mark:'
    path: src/dfa.rs
    title: src/dfa.rs
  - icon: ':heavy_check_mark:'
    path: src/fxhash.rs
    title: src/fxhash.rs
  - icon: ':heavy_check_mark:'
    path: src/int.rs
    title: src/int.rs
  - icon: ':heavy_check_mark:'
    path: src/io.rs
    title: src/io.rs
  - icon: ':heavy_check_mark:'
    path: src/mint.rs
    title: src/mint.rs
  - icon: ':heavy_check_mark:'
    path: src/mint/io.rs
    title: src/mint/io.rs
  - icon: ':heavy_check_mark:'
    path: src/num.rs
    title: src/num.rs
  - icon: ':heavy_check_mark:'
    path: src/rand/seed.rs
    title: src/rand/seed.rs
  - icon: ':heavy_check_mark:'
    path: src/zo.rs
    title: src/zo.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=0570
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: test/src/bin/dfa_test.rs\n"
  code: "// verification-helper: PROBLEM http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=0570\n\
    \nuse lib::dfa::*;\nuse lib::io::*;\nuse lib::mint::io::*;\n\n#[derive(Ord, PartialOrd,\
    \ Eq, PartialEq, Copy, Clone, Hash)]\nenum ZigZagState {\n    Initial,\n    First(u8),\n\
    \    Up(u8),\n    Down(u8),\n}\n\nstruct ZigZag;\n\nimpl Dfa for ZigZag {\n  \
    \  type Alphabet = u8;\n    type State = Option<ZigZagState>;\n    fn init(&self)\
    \ -> Self::State {\n        Some(ZigZagState::Initial)\n    }\n    fn next(&self,\
    \ s: Self::State, a: Self::Alphabet, _: usize) -> Self::State {\n        use ZigZagState::*;\n\
    \        if let Some(s) = s {\n            match s {\n                Initial\
    \ if a == b'0' => Some(Initial),\n                Initial => Some(First(a)),\n\
    \                First(d) if d < a => Some(Up(a)),\n                First(d) if\
    \ d > a => Some(Down(a)),\n                Up(d) if d > a => Some(Down(a)),\n\
    \                Down(d) if d < a => Some(Up(a)),\n                _ => None,\n\
    \            }\n        } else {\n            None\n        }\n    }\n    fn accept(&self,\
    \ s: Self::State) -> bool {\n        s.is_some()\n    }\n    fn unsuccessful(&self,\
    \ s: Self::State) -> bool {\n        s.is_none()\n    }\n}\n\n#[derive(Default,\
    \ Clone, Copy, PartialEq, Eq)]\nstruct Modx;\n\nimpl Mod for Modx {\n    const\
    \ M: u32 = 10000;\n    const PHI: u32 = 4000;\n}\n\nfn main() {\n    let mut io\
    \ = IO::new();\n    let [a0, b]: [&[u8]; 2] = io.scan();\n    let m = io.scan();\n\
    \    let mut a = vec![b'0'; b.len()];\n    a[b.len() - a0.len()..].copy_from_slice(&a0);\n\
    \n    let dfa = And(ZigZag, And(MultipleOf(m), And(Leq(&b), Not(Lt(&a)))));\n\
    \    let alphabet = \"0123456789\".as_bytes();\n    let ans: Mint<Modx> = dfa.count(a.len(),\
    \ alphabet);\n    println!(\"{}\", ans);\n}\n"
  dependsOn:
  - src/bits.rs
  - src/bounded.rs
  - src/cast.rs
  - src/dfa.rs
  - src/fxhash.rs
  - src/int.rs
  - src/io.rs
  - src/mint.rs
  - src/mint/io.rs
  - src/num.rs
  - src/rand/seed.rs
  - src/zo.rs
  isVerificationFile: true
  path: test/src/bin/dfa_test.rs
  requiredBy: []
  timestamp: '2021-02-10 04:47:06+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: test/src/bin/dfa_test.rs
layout: document
redirect_from:
- /verify/test/src/bin/dfa_test.rs
- /verify/test/src/bin/dfa_test.rs.html
title: test/src/bin/dfa_test.rs
---
