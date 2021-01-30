---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/conv.rs
    title: src/conv.rs
  - icon: ':heavy_check_mark:'
    path: src/ds/uvec.rs
    title: src/ds/uvec.rs
  - icon: ':question:'
    path: src/io.rs
    title: src/io.rs
  - icon: ':question:'
    path: src/mint.rs
    title: src/mint.rs
  - icon: ':question:'
    path: src/zo.rs
    title: src/zo.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/ntt_mint_garner_test.rs
    title: test/src/bin/ntt_mint_garner_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/ntt_mint_test.rs
    title: test/src/bin/ntt_mint_test.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/mint/conv.rs\n"
  code: "pub use super::*;\npub use crate::conv::*;\npub use crate::ds::uvec::*;\n\
    \nmacro_rules! impl_ntt {\n\t($module:ident, $modu:ty, $prim:expr) => {\n\t\t\
    pub mod $module {\n\t\t\tuse super::*;\n\n\t\t\ttype FpType = Mint<$modu>;\n\n\
    \t\t\tstatic mut ROOT: UVec<FpType> = UVec(Vec::new());\n\t\t\tstatic mut INV_ROOT:\
    \ UVec<FpType> = UVec(Vec::new());\n\n\t\t\t/// reserve for n up to 2^k\n\t\t\t\
    pub fn reserve(k: usize) {\n\t\t\t\tunsafe {\n\t\t\t\t\tif k <= ROOT.len() {\n\
    \t\t\t\t\t\treturn;\n\t\t\t\t\t}\n\t\t\t\t\tROOT.resize(k, Default::default());\n\
    \t\t\t\t\tINV_ROOT.resize(k, Default::default());\n\t\t\t\t\tlet m = FpType::M\
    \ - 1;\n\t\t\t\t\tlet proot = FpType::from($prim);\n\t\t\t\t\tfor i in 0..k {\n\
    \t\t\t\t\t\tROOT[i] = -proot.pow(m >> (i + 2));\n\t\t\t\t\t\tINV_ROOT[i] = ROOT[i].inv();\n\
    \t\t\t\t\t}\n\t\t\t\t}\n\t\t\t}\n\n\t\t\tpub fn ntt(a: &mut UVec<FpType>) {\n\t\
    \t\t\tlet n = a.len();\n\t\t\t\tlet mut m = n >> 1;\n\t\t\t\twhile m != 0 {\n\t\
    \t\t\t\tlet mut w = FpType::ONE;\n\t\t\t\t\tfor (k, t) in (0..n).step_by(m * 2).zip(1_u32..)\
    \ {\n\t\t\t\t\t\tfor i in 0..m {\n\t\t\t\t\t\t\tlet u = a[k + i];\n\t\t\t\t\t\t\
    \tlet v = a[k + i + m] * w;\n\t\t\t\t\t\t\ta[k + i] = u + v;\n\t\t\t\t\t\t\ta[k\
    \ + i + m] = u - v;\n\t\t\t\t\t\t}\n\t\t\t\t\t\tw *= unsafe { ROOT[t.trailing_zeros()\
    \ as usize] };\n\t\t\t\t\t}\n\t\t\t\t\tm >>= 1;\n\t\t\t\t}\n\t\t\t}\n\n\t\t\t\
    pub fn inv_ntt(a: &mut UVec<FpType>) {\n\t\t\t\tlet n = a.len();\n\t\t\t\tlet\
    \ mut m = 1;\n\t\t\t\twhile m < n {\n\t\t\t\t\tlet mut w = FpType::ONE;\n\t\t\t\
    \t\tfor (k, t) in (0..n).step_by(m * 2).zip(1_u32..) {\n\t\t\t\t\t\tfor i in 0..m\
    \ {\n\t\t\t\t\t\t\tlet u = a[k + i];\n\t\t\t\t\t\t\tlet v = a[k + i + m];\n\t\t\
    \t\t\t\t\ta[k + i] = u + v;\n\t\t\t\t\t\t\ta[k + i + m] = (u - v) * w;\n\t\t\t\
    \t\t\t}\n\t\t\t\t\t\tw *= unsafe { INV_ROOT[t.trailing_zeros() as usize] };\n\t\
    \t\t\t\t}\n\t\t\t\t\tm <<= 1;\n\t\t\t\t}\n\t\t\t\tlet d = FpType::from(n).inv();\n\
    \t\t\t\ta.iter_mut().for_each(|e| *e *= d);\n\t\t\t}\n\n\t\t\tpub fn conv<'a,\
    \ 'b>(a: &'a mut UVec<FpType>, b: &'b mut UVec<FpType>) {\n\t\t\t\tlet len = a.len()\
    \ + b.len() - 1;\n\t\t\t\tfn ilog2(n: usize) -> u32 {\n\t\t\t\t\tstd::mem::size_of::<usize>()\
    \ as u32 * 8 - n.leading_zeros() - 1\n\t\t\t\t}\n\t\t\t\tlet n: usize = 1 << ilog2(len\
    \ * 2 - 1);\n\t\t\t\treserve(n.trailing_zeros() as usize);\n\t\t\t\ta.resize(n,\
    \ Default::default());\n\t\t\t\tb.resize(n, Default::default());\n\t\t\t\tntt(a);\n\
    \t\t\t\tntt(b);\n\t\t\t\ta.iter_mut().zip(b.iter()).for_each(|(a, b)| *a *= *b);\n\
    \t\t\t\tb.clear();\n\t\t\t\tinv_ntt(a);\n\t\t\t\ta.truncate(len);\n\t\t\t}\n\n\
    \t\t\timpl Conv for FpType {\n\t\t\t\tfn conv(mut lhs: Vec<Self>, mut rhs: Vec<Self>)\
    \ -> Vec<Self> {\n\t\t\t\t\tconv(lhs.as_mut(), rhs.as_mut());\n\t\t\t\t\tlhs\n\
    \t\t\t\t}\n\t\t\t\tfn conv_in_place(lhs: &mut Vec<Self>, rhs: &mut Vec<Self>)\
    \ {\n\t\t\t\t\tconv(lhs.as_mut(), rhs.as_mut());\n\t\t\t\t}\n\t\t\t}\n\t\t}\n\t\
    };\n}\n\nimpl_ntt!(impl_b, ModB, 3);\nimpl_ntt!(impl_c, ModC, 5);\nimpl_ntt!(impl_d,\
    \ ModD, 5);\n\nimpl Conv for Mint17 {\n\tfn conv_in_place(lhs: &mut Vec<Self>,\
    \ rhs: &mut Vec<Self>) {\n\t\tlet r12 = MintC::from(MintB::M).inv();\n\t\tlet\
    \ r13 = MintD::from(MintB::M).inv();\n\t\tlet r23 = MintD::from(MintC::M).inv();\n\
    \t\tfn run<M: Mod>(lhs: &mut Vec<Mint17>, rhs: &mut Vec<Mint17>) -> Vec<Mint<M>>\n\
    \t\twhere\n\t\t\tMint<M>: Conv,\n\t\t{\n\t\t\tlet lhs = lhs.iter().map(|&e| Mint::from(e.value())).collect();\n\
    \t\t\tlet rhs = rhs.iter().map(|&e| Mint::from(e.value())).collect();\n\t\t\t\
    Conv::conv(lhs, rhs)\n\t\t}\n\t\tlet v1: Vec<MintB> = run(lhs, rhs);\n\t\tlet\
    \ v2: Vec<MintC> = run(lhs, rhs);\n\t\tlet v3: Vec<MintD> = run(lhs, rhs);\n\t\
    \tlhs.resize(v1.len(), Default::default());\n\t\tfor (((e0, e1), e2), e3) in lhs.iter_mut().zip(v1).zip(v2).zip(v3)\
    \ {\n\t\t\tlet x1 = e1;\n\t\t\tlet x2 = (e2 - x1.value()) * r12;\n\t\t\tlet x3\
    \ = ((e3 - x1.value()) * r13 - x2.value()) * r23;\n\t\t\tlet mut x = MintA::from(x1.value());\n\
    \t\t\tx += MintA::from(x2.value()) * MintB::M;\n\t\t\tx += MintA::from(x3.value())\
    \ * MintB::M * MintC::M;\n\t\t\t*e0 = x.value().into();\n\t\t}\n\t}\n}\n"
  dependsOn:
  - src/conv.rs
  - src/ds/uvec.rs
  - src/io.rs
  - src/mint.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/mint/conv.rs
  requiredBy: []
  timestamp: '2021-01-30 12:54:22+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/ntt_mint_test.rs
  - test/src/bin/ntt_mint_garner_test.rs
documentation_of: src/mint/conv.rs
layout: document
redirect_from:
- /library/src/mint/conv.rs
- /library/src/mint/conv.rs.html
title: src/mint/conv.rs
---
