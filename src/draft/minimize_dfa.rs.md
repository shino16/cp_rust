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
  code: "fn get_q() -> Vec<Vec<bool>> {\n\tunimplemented!()\n}\n\nfn next_state(mut\
    \ s: Vec<bool>, op: Op) -> Vec<bool> {\n\tunimplemented!()\n}\n\nfn is_accepted(v:\
    \ &Vec<bool>) -> bool {\n\tunimplemented!()\n}\n\nfn run<F, G, H, State: Clone\
    \ + Ord, Alphabet: Clone>(\n\tget_q: F,\n\tis_accepted: G,\n\tnext_state: H,\n\
    \talphabet: &[Alphabet],\n) -> Vec<BTreeSet<State>>\nwhere\n\tF: FnOnce() -> Vec<State>,\n\
    \tG: Fn(&State) -> bool,\n\tH: Fn(State, Alphabet) -> State,\n{\n\tlet q = get_q();\n\
    \tlet (f, notf) = q\n\t\t.iter()\n\t\t.cloned()\n\t\t.partition::<BTreeSet<_>,\
    \ _>(|v| is_accepted(v));\n\n\tlet mut p = vec![f.clone(), notf.clone()];\n\t\
    let mut p2 = Vec::new();\n\tlet mut w = vec![f, notf];\n\twhile let Some(a) =\
    \ w.pop() {\n\t\tfor op in alphabet {\n\t\t\tlet x: BTreeSet<_> = q\n\t\t\t\t\
    .iter()\n\t\t\t\t.cloned()\n\t\t\t\t.filter(|s| a.get(&next_state(s.clone(), op.clone())).is_some())\n\
    \t\t\t\t.collect();\n\t\t\tfor y in p.drain(..) {\n\t\t\t\tlet inter = x.intersection(&y);\n\
    \t\t\t\tlet inter: BTreeSet<_> = inter.cloned().collect();\n\t\t\t\tlet imply\
    \ = y.difference(&x);\n\t\t\t\tlet imply: BTreeSet<_> = imply.cloned().collect();\n\
    \t\t\t\tif inter.is_empty() || imply.is_empty() {\n\t\t\t\t\tp2.push(y);\n\t\t\
    \t\t\tcontinue;\n\t\t\t\t}\n\t\t\t\tp2.push(inter.clone());\n\t\t\t\tp2.push(imply.clone());\n\
    \t\t\t\tif let Some((i, _)) = w.iter().enumerate().filter(|&(_, s)| s == &y).next()\
    \ {\n\t\t\t\t\tw.swap_remove(i);\n\t\t\t\t\tw.push(inter);\n\t\t\t\t\tw.push(imply);\n\
    \t\t\t\t} else if inter.len() <= imply.len() {\n\t\t\t\t\tw.push(inter);\n\t\t\
    \t\t} else {\n\t\t\t\t\tw.push(imply);\n\t\t\t\t}\n\t\t\t}\n\t\t\tstd::mem::swap(&mut\
    \ p, &mut p2);\n\t\t}\n\t}\n\tp.sort_unstable();\n\tp\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/draft/minimize_dfa.rs
  requiredBy: []
  timestamp: '2020-11-27 14:24:44+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/draft/minimize_dfa.rs
layout: document
redirect_from:
- /library/src/draft/minimize_dfa.rs
- /library/src/draft/minimize_dfa.rs.html
title: src/draft/minimize_dfa.rs
---
