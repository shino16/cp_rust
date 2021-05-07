---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: src/tests.rs
    title: src/tests.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/cargo_test.rs
    title: test/src/bin/cargo_test.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.5/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.5/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/slice/sort.rs\n"
  code: "use std::mem::MaybeUninit;\n\npub fn count_sort_bytes(slice: &[u8], out:\
    \ &mut Vec<u8>, max_key: usize) {\n    count_sort(slice, out, max_key, |x| x as\
    \ usize);\n}\n\npub fn count_sort<T: Copy>(\n    slice: &[T],\n    out: &mut Vec<T>,\n\
    \    max_key: usize,\n    mut key: impl FnMut(T) -> usize,\n) {\n    let mut count\
    \ = vec![0; max_key + 1];\n    for &e in slice {\n        count[key(e)] += 1;\n\
    \    }\n    for i in 0..max_key {\n        count[i + 1] += count[i];\n    }\n\
    \    out.clear();\n    out.reserve(slice.len());\n    {\n        // SAFETY: https://docs.rs/uninit/0.4.0/uninit/extension_traits/trait.VecCapacity.html#method.get_backing_buffer_with_leaking_writes\n\
    \        let out: &mut [MaybeUninit<T>] =\n            unsafe { std::slice::from_raw_parts_mut(out.as_mut_ptr()\
    \ as _, slice.len()) };\n        for &e in slice.iter().rev() {\n            count[key(e)]\
    \ -= 1;\n            out[count[key(e)]] = MaybeUninit::new(e.clone());\n     \
    \   }\n    }\n    unsafe {\n        out.set_len(slice.len());\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/slice/sort.rs
  requiredBy:
  - src/tests.rs
  timestamp: '2021-03-31 15:51:17+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/cargo_test.rs
documentation_of: src/slice/sort.rs
layout: document
redirect_from:
- /library/src/slice/sort.rs
- /library/src/slice/sort.rs.html
title: src/slice/sort.rs
---
