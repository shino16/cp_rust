---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/bit.rs
    title: src/bit.rs
  - icon: ':heavy_check_mark:'
    path: src/cast.rs
    title: src/cast.rs
  - icon: ':heavy_check_mark:'
    path: src/fxhash.rs
    title: src/fxhash.rs
  - icon: ':heavy_check_mark:'
    path: src/int.rs
    title: src/int.rs
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
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/dfa_test.rs
    title: test/src/bin/dfa_test.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/dfa.rs\n"
  code: "use crate::fxhash::FxHashMap as HashMap;\nuse crate::int::*;\nuse std::cmp::Ordering;\n\
    use std::hash::Hash;\n\npub const DIGITS: &[u8] = b\"0123456789\";\n\npub trait\
    \ Dfa {\n\ttype Alphabet;\n\ttype State;\n\tfn init(&self) -> Self::State;\n\t\
    fn next(&self, s: Self::State, a: Self::Alphabet, i: usize) -> Self::State;\n\t\
    fn accept(&self, s: Self::State) -> bool;\n\tfn successful(&self, _: Self::State)\
    \ -> bool {\n\t\tfalse\n\t}\n\tfn unsuccessful(&self, _: Self::State) -> bool\
    \ {\n\t\tfalse\n\t}\n\n\tfn count<I: Num>(&self, n: usize, alphabet: &[Self::Alphabet])\
    \ -> I\n\twhere\n\t\tSelf::Alphabet: Copy,\n\t\tSelf::State: Eq + Hash + Copy,\n\
    \t{\n\t\tcount(self, n, alphabet)\n\t}\n}\n\npub struct And<X, Y>(pub X, pub Y);\n\
    \nimpl<X: Dfa<Alphabet = A>, Y: Dfa<Alphabet = A>, A: Copy> Dfa for And<X, Y>\
    \ {\n\ttype Alphabet = A;\n\ttype State = (X::State, Y::State);\n\tfn init(&self)\
    \ -> Self::State {\n\t\t(self.0.init(), self.1.init())\n\t}\n\tfn next(&self,\
    \ (s0, s1): Self::State, a: Self::Alphabet, i: usize) -> Self::State {\n\t\t(self.0.next(s0,\
    \ a, i), self.1.next(s1, a, i))\n\t}\n\tfn accept(&self, (s0, s1): Self::State)\
    \ -> bool {\n\t\tself.0.accept(s0) && self.1.accept(s1)\n\t}\n\tfn successful(&self,\
    \ (s0, s1): Self::State) -> bool {\n\t\tself.0.successful(s0) && self.1.successful(s1)\n\
    \t}\n\tfn unsuccessful(&self, (s0, s1): Self::State) -> bool {\n\t\tself.0.unsuccessful(s0)\
    \ || self.1.unsuccessful(s1)\n\t}\n}\n\npub struct Prod<X, Y>(pub X, pub Y);\n\
    \nimpl<X: Dfa, Y: Dfa> Dfa for Prod<X, Y> {\n\ttype Alphabet = (X::Alphabet, Y::Alphabet);\n\
    \ttype State = (X::State, Y::State);\n\tfn init(&self) -> Self::State {\n\t\t\
    (self.0.init(), self.1.init())\n\t}\n\tfn next(&self, (s0, s1): Self::State, (a0,\
    \ a1): Self::Alphabet, i: usize) -> Self::State {\n\t\t(self.0.next(s0, a0, i),\
    \ self.1.next(s1, a1, i))\n\t}\n\tfn accept(&self, (s0, s1): Self::State) -> bool\
    \ {\n\t\tself.0.accept(s0) && self.1.accept(s1)\n\t}\n\tfn successful(&self, (s0,\
    \ s1): Self::State) -> bool {\n\t\tself.0.successful(s0) && self.1.successful(s1)\n\
    \t}\n\tfn unsuccessful(&self, (s0, s1): Self::State) -> bool {\n\t\tself.0.unsuccessful(s0)\
    \ || self.1.unsuccessful(s1)\n\t}\n}\n\npub struct Not<X>(pub X);\n\nimpl<X: Dfa>\
    \ Dfa for Not<X> {\n\ttype Alphabet = X::Alphabet;\n\ttype State = X::State;\n\
    \tfn init(&self) -> Self::State {\n\t\tself.0.init()\n\t}\n\tfn next(&self, s:\
    \ Self::State, a: Self::Alphabet, i: usize) -> Self::State {\n\t\tself.0.next(s,\
    \ a, i)\n\t}\n\tfn accept(&self, s: Self::State) -> bool {\n\t\t!self.0.accept(s)\n\
    \t}\n\tfn successful(&self, s: Self::State) -> bool {\n\t\tself.0.unsuccessful(s)\n\
    \t}\n\tfn unsuccessful(&self, s: Self::State) -> bool {\n\t\tself.0.successful(s)\n\
    \t}\n}\n\npub struct Lt<'a>(pub &'a [u8]);\n\nimpl Dfa for Lt<'_> {\n\ttype Alphabet\
    \ = u8;\n\ttype State = Ordering;\n\tfn init(&self) -> Self::State {\n\t\tOrdering::Equal\n\
    \t}\n\tfn next(&self, s: Self::State, a: Self::Alphabet, i: usize) -> Self::State\
    \ {\n\t\ts.then(a.cmp(&self.0[i]))\n\t}\n\tfn accept(&self, s: Self::State) ->\
    \ bool {\n\t\ts == Ordering::Less\n\t}\n\tfn successful(&self, s: Self::State)\
    \ -> bool {\n\t\ts == Ordering::Less\n\t}\n\tfn unsuccessful(&self, s: Self::State)\
    \ -> bool {\n\t\ts == Ordering::Greater\n\t}\n}\n\npub struct Leq<'a>(pub &'a\
    \ [u8]);\n\nimpl Dfa for Leq<'_> {\n\ttype Alphabet = u8;\n\ttype State = Ordering;\n\
    \tfn init(&self) -> Self::State {\n\t\tOrdering::Equal\n\t}\n\tfn next(&self,\
    \ s: Self::State, a: Self::Alphabet, i: usize) -> Self::State {\n\t\ts.then(a.cmp(&self.0[i]))\n\
    \t}\n\tfn accept(&self, s: Self::State) -> bool {\n\t\ts != Ordering::Greater\n\
    \t}\n\tfn successful(&self, s: Self::State) -> bool {\n\t\ts == Ordering::Less\n\
    \t}\n\tfn unsuccessful(&self, s: Self::State) -> bool {\n\t\ts == Ordering::Greater\n\
    \t}\n}\n\npub struct MultipleOf(pub u32);\n\nimpl Dfa for MultipleOf {\n\ttype\
    \ Alphabet = u8;\n\ttype State = u32;\n\tfn init(&self) -> Self::State {\n\t\t\
    0\n\t}\n\tfn next(&self, s: Self::State, a: Self::Alphabet, _: usize) -> Self::State\
    \ {\n\t\t(s * 10 + (a - b'0') as u32) % self.0\n\t}\n\tfn accept(&self, s: Self::State)\
    \ -> bool {\n\t\ts == 0\n\t}\n}\n\nfn count<I: Num, X: Dfa + ?Sized>(dfa: &X,\
    \ n: usize, alphabet: &[X::Alphabet]) -> I\nwhere\n\tX::Alphabet: Copy,\n\tX::State:\
    \ Eq + Hash + Copy,\n{\n\tlet mut dp = HashMap::default();\n\tlet mut dp2 = HashMap::default();\n\
    \tdp.insert(dfa.init(), I::ONE);\n\tfor i in 0..n {\n\t\tdp2.clear();\n\t\tfor\
    \ (s, k) in dp.drain() {\n\t\t\tfor &a in alphabet {\n\t\t\t\tlet s1 = dfa.next(s,\
    \ a, i);\n\t\t\t\tif dfa.unsuccessful(s1) {\n\t\t\t\t\tcontinue;\n\t\t\t\t}\n\t\
    \t\t\t*dp2.entry(s1).or_insert(I::ZERO) += k;\n\t\t\t}\n\t\t}\n\t\tstd::mem::swap(&mut\
    \ dp, &mut dp2);\n\t}\n\tlet mut sum = I::ZERO;\n\tfor (s, k) in dp {\n\t\tif\
    \ dfa.accept(s) {\n\t\t\tsum += k;\n\t\t}\n\t}\n\tsum\n}\n"
  dependsOn:
  - src/bit.rs
  - src/cast.rs
  - src/fxhash.rs
  - src/int.rs
  - src/num.rs
  - src/rand/seed.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/dfa.rs
  requiredBy: []
  timestamp: '2021-01-27 17:46:37+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/dfa_test.rs
documentation_of: src/dfa.rs
layout: document
redirect_from:
- /library/src/dfa.rs
- /library/src/dfa.rs.html
title: src/dfa.rs
---
