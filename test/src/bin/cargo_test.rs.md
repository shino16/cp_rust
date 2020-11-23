---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: src/as_int.rs
    title: src/as_int.rs
  - icon: ':question:'
    path: src/bit.rs
    title: src/bit.rs
  - icon: ':question:'
    path: src/conv.rs
    title: src/conv.rs
  - icon: ':question:'
    path: src/ds/uvec.rs
    title: src/ds/uvec.rs
  - icon: ':question:'
    path: src/fp.rs
    title: src/fp.rs
  - icon: ':question:'
    path: src/fp/conv.rs
    title: src/fp/conv.rs
  - icon: ':question:'
    path: src/io.rs
    title: src/io.rs
  - icon: ':question:'
    path: src/iter.rs
    title: src/iter.rs
  - icon: ':question:'
    path: src/iter/prod.rs
    title: src/iter/prod.rs
  - icon: ':question:'
    path: src/make_vec.rs
    title: src/make_vec.rs
  - icon: ':question:'
    path: src/math/gcd.rs
    title: src/math/gcd.rs
  - icon: ':question:'
    path: src/mint.rs
    title: src/mint.rs
  - icon: ':question:'
    path: src/mint/conv.rs
    title: src/mint/conv.rs
  - icon: ':question:'
    path: src/num.rs
    title: src/num.rs
  - icon: ':question:'
    path: src/rng.rs
    title: src/rng.rs
  - icon: ':question:'
    path: src/tests.rs
    title: src/tests.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_1_A
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "// verify-helper: PROBLEM http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_1_A\n\
    \n#[allow(unused_imports)]\nuse lib::tests::*;\n\nuse std::io::{self, Write, Result};\n\
    use std::process::Command;\n\nfn main() -> Result<()> {\n    let output = Command::new(\"\
    cargo\")\n        .arg(\"test\")\n        .arg(\"--\")\n        .arg(\"--test-threads=1\"\
    )\n        .output()?;\n\n    if !output.status.success() {\n        println!(\"\
    `cargo test` failed\");\n        println!(\"--- captured stdout ---\");\n    \
    \    io::stdout().write_all(&output.stdout)?;\n        println!(\"--- captured\
    \ stderr ---\");\n        io::stdout().write_all(&output.stderr)?;\n    }\n\n\
    \    println!(\"Hello World\");\n\n    Ok(())\n}\n"
  dependsOn:
  - src/as_int.rs
  - src/bit.rs
  - src/conv.rs
  - src/ds/uvec.rs
  - src/fp.rs
  - src/fp/conv.rs
  - src/io.rs
  - src/iter.rs
  - src/iter/prod.rs
  - src/make_vec.rs
  - src/math/gcd.rs
  - src/mint.rs
  - src/mint/conv.rs
  - src/num.rs
  - src/rng.rs
  - src/tests.rs
  isVerificationFile: true
  path: test/src/bin/cargo_test.rs
  requiredBy: []
  timestamp: '2020-11-24 01:55:32+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: test/src/bin/cargo_test.rs
layout: document
redirect_from:
- /verify/test/src/bin/cargo_test.rs
- /verify/test/src/bin/cargo_test.rs.html
title: test/src/bin/cargo_test.rs
---
