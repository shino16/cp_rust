---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/bits.rs
    title: src/bits.rs
  - icon: ':question:'
    path: src/bounded.rs
    title: src/bounded.rs
  - icon: ':heavy_check_mark:'
    path: src/cast.rs
    title: src/cast.rs
  - icon: ':heavy_check_mark:'
    path: src/fp.rs
    title: src/fp.rs
  - icon: ':heavy_check_mark:'
    path: src/func/memo.rs
    title: src/func/memo.rs
  - icon: ':heavy_check_mark:'
    path: src/int.rs
    title: src/int.rs
  - icon: ':heavy_check_mark:'
    path: src/int/gcd.rs
    title: src/int/gcd.rs
  - icon: ':heavy_check_mark:'
    path: src/iter.rs
    title: src/iter.rs
  - icon: ':heavy_check_mark:'
    path: src/iter/prod.rs
    title: src/iter/prod.rs
  - icon: ':heavy_check_mark:'
    path: src/make_vec.rs
    title: src/make_vec.rs
  - icon: ':heavy_check_mark:'
    path: src/mint.rs
    title: src/mint.rs
  - icon: ':question:'
    path: src/num.rs
    title: src/num.rs
  - icon: ':heavy_check_mark:'
    path: src/rand/seed.rs
    title: src/rand/seed.rs
  - icon: ':heavy_check_mark:'
    path: src/rand/xoshiro256plus.rs
    title: src/rand/xoshiro256plus.rs
  - icon: ':heavy_check_mark:'
    path: src/slice/lcp.rs
    title: src/slice/lcp.rs
  - icon: ':heavy_check_mark:'
    path: src/slice/perm.rs
    title: src/slice/perm.rs
  - icon: ':heavy_check_mark:'
    path: src/slice/sa.rs
    title: src/slice/sa.rs
  - icon: ':heavy_check_mark:'
    path: src/slice/sort.rs
    title: src/slice/sort.rs
  - icon: ':heavy_check_mark:'
    path: src/tests.rs
    title: src/tests.rs
  - icon: ':question:'
    path: src/util/trait_alias.rs
    title: src/util/trait_alias.rs
  - icon: ':question:'
    path: src/zo.rs
    title: src/zo.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_1_A
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: test/src/bin/cargo_test.rs\n"
  code: "// verification-helper: PROBLEM http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_1_A\n\
    \n#[allow(unused_imports)]\nuse lib::tests::*;\n\nuse std::io::{self, Write, Result};\n\
    use std::process::Command;\n\nfn main() -> Result<()> {\n    let output = Command::new(\"\
    cargo\")\n        .arg(\"test\")\n        .arg(\"--release\")\n        .arg(\"\
    --\")\n        .arg(\"--test-threads=1\")\n        .output()?;\n\n    if !output.status.success()\
    \ {\n        eprintln!(\"`cargo test` failed\");\n        eprintln!(\"--- captured\
    \ stdout ---\");\n        io::stdout().write_all(&output.stdout)?;\n        eprintln!(\"\
    --- captured stderr ---\");\n        io::stdout().write_all(&output.stderr)?;\n\
    \    }\n\n    println!(\"Hello World\");\n\n    Ok(())\n}\n"
  dependsOn:
  - src/bits.rs
  - src/bounded.rs
  - src/cast.rs
  - src/fp.rs
  - src/func/memo.rs
  - src/int.rs
  - src/int/gcd.rs
  - src/iter.rs
  - src/iter/prod.rs
  - src/make_vec.rs
  - src/mint.rs
  - src/num.rs
  - src/rand/seed.rs
  - src/rand/xoshiro256plus.rs
  - src/slice/lcp.rs
  - src/slice/perm.rs
  - src/slice/sa.rs
  - src/slice/sort.rs
  - src/tests.rs
  - src/util/trait_alias.rs
  - src/zo.rs
  isVerificationFile: true
  path: test/src/bin/cargo_test.rs
  requiredBy: []
  timestamp: '2021-03-14 05:03:05+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: test/src/bin/cargo_test.rs
layout: document
redirect_from:
- /verify/test/src/bin/cargo_test.rs
- /verify/test/src/bin/cargo_test.rs.html
title: test/src/bin/cargo_test.rs
---
