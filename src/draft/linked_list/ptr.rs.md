---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: src/draft/linked_list.rs
    title: src/draft/linked_list.rs
  - icon: ':warning:'
    path: src/ds.rs
    title: src/ds.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/draft/linked_list/ptr.rs\n"
  code: "pub use super::*;\n\n/// FIXME: double free\n/// https://github.com/shino16/cpr/runs/1796042987?check_suite_focus=true#step:8:65\n\
    #[derive(Clone)]\npub struct CursorPtr<T> {\n\tat: NonNull<Node<T>>,\n\tlist:\
    \ NonNull<LinkedList<T>>,\n}\n\nimpl<T> LinkedList<T> {\n\tpub fn begin_ptr<'a>(&mut\
    \ self) -> CursorPtr<T> {\n\t\tCursorPtr { at: self.head, list: self.into() }\n\
    \t}\n\tpub fn end_ptr<'a>(&mut self) -> CursorPtr<T> {\n\t\tCursorPtr { at: self.tail,\
    \ list: self.into() }\n\t}\n}\n\nimpl<T> Deref for CursorPtr<T> {\n\ttype Target\
    \ = T;\n\tfn deref(&self) -> &Self::Target {\n\t\tunsafe { &self.at.as_ref().next_val.as_ref().unwrap().1\
    \ }\n\t}\n}\n\nimpl<T> DerefMut for CursorPtr<T> {\n\tfn deref_mut(&mut self)\
    \ -> &mut Self::Target {\n\t\tunsafe { &mut self.at.as_mut().next_val.as_mut().unwrap().1\
    \ }\n\t}\n}\n\nimpl<T> CursorPtr<T> {\n\tpub fn dangling() -> Self {\n\t\tSelf\
    \ { at: NonNull::dangling(), list: NonNull::dangling() }\n\t}\n\tpub fn next(&mut\
    \ self) -> Option<&mut Self> {\n\t\tself.at = unsafe { self.at.as_ref() }.next_val.as_ref()?.0;\n\
    \t\tSome(self)\n\t}\n\tpub fn prev(&mut self) -> Option<&mut Self> {\n\t\tself.at\
    \ = unsafe { self.at.as_ref() }.prev?;\n\t\tSome(self)\n\t}\n\tpub fn advance(&mut\
    \ self, by: isize) -> Option<&mut Self> {\n\t\tif by >= 0 {\n\t\t\tfor _ in 0..by\
    \ {\n\t\t\t\tself.next()?;\n\t\t\t}\n\t\t} else {\n\t\t\tfor _ in by..0 {\n\t\t\
    \t\tself.prev()?;\n\t\t\t}\n\t\t}\n\t\tSome(self)\n\t}\n\t/// safety: must not\
    \ alias list, prev, at, next\n\tpub unsafe fn insert(&mut self, val: T) -> Self\
    \ {\n\t\tlet prev = self.at.as_ref().prev;\n\t\tlet new_node =\n\t\t\tself.list.as_mut().new_node(Node\
    \ { prev, next_val: Some((self.at, val)) });\n\t\tself.at.as_mut().prev = Some(new_node);\n\
    \t\tif let Some(mut prev) = prev {\n\t\t\tprev.as_mut().next_val.as_mut().unwrap().0\
    \ = new_node;\n\t\t} else {\n\t\t\tself.list.as_mut().head = new_node;\n\t\t}\n\
    \t\tself.at = new_node;\n\t\tself.list.as_mut().len += 1;\n\t\tlet next = new_node.as_ref().next_val.as_ref().map(|t|\
    \ t.0);\n\t\tassert_ne!(Some(new_node), next);\n\t\tSelf { at: new_node, list:\
    \ self.list }\n\t}\n\t/// safety: must not alias list, prev, at, next\n\tpub unsafe\
    \ fn remove(&mut self) -> Option<T> {\n\t\tif self.at == self.list.as_mut().tail\
    \ {\n\t\t\treturn None;\n\t\t}\n\t\tlet node = std::ptr::read(self.at.as_ptr());\n\
    \t\tlet (mut next, val) = node.next_val?;\n\t\tif let Some(mut prev) = node.prev\
    \ {\n\t\t\t*next.as_mut().prev.as_mut().unwrap() = prev;\n\t\t\tprev.as_mut().next_val.as_mut().unwrap().0\
    \ = next;\n\t\t} else {\n\t\t\tnext.as_mut().prev = None;\n\t\t\tself.list.as_mut().head\
    \ = next;\n\t\t}\n\t\tself.at = next;\n\t\tself.list.as_mut().len -= 1;\n\t\t\
    Some(val)\n\t}\n}\n"
  dependsOn:
  - src/draft/linked_list.rs
  - src/ds.rs
  isVerificationFile: false
  path: src/draft/linked_list/ptr.rs
  requiredBy: []
  timestamp: '2021-01-30 17:33:56+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/draft/linked_list/ptr.rs
layout: document
redirect_from:
- /library/src/draft/linked_list/ptr.rs
- /library/src/draft/linked_list/ptr.rs.html
title: src/draft/linked_list/ptr.rs
---
