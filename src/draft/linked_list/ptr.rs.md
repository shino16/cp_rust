---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: src/draft/linked_list.rs
    title: src/draft/linked_list.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.5/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.5/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/draft/linked_list/ptr.rs\n"
  code: "pub use super::*;\n\n/// FIXME: double free\n/// https://github.com/shino16/cpr/runs/1796042987?check_suite_focus=true#step:8:65\n\
    #[derive(Clone)]\npub struct CursorPtr<T> {\n    at: NonNull<Node<T>>,\n    list:\
    \ NonNull<LinkedList<T>>,\n}\n\nimpl<T> LinkedList<T> {\n    pub fn begin_ptr<'a>(&mut\
    \ self) -> CursorPtr<T> {\n        CursorPtr { at: self.head, list: self.into()\
    \ }\n    }\n    pub fn end_ptr<'a>(&mut self) -> CursorPtr<T> {\n        CursorPtr\
    \ { at: self.tail, list: self.into() }\n    }\n}\n\nimpl<T> Deref for CursorPtr<T>\
    \ {\n    type Target = T;\n    fn deref(&self) -> &Self::Target {\n        unsafe\
    \ { &self.at.as_ref().next_val.as_ref().unwrap().1 }\n    }\n}\n\nimpl<T> DerefMut\
    \ for CursorPtr<T> {\n    fn deref_mut(&mut self) -> &mut Self::Target {\n   \
    \     unsafe { &mut self.at.as_mut().next_val.as_mut().unwrap().1 }\n    }\n}\n\
    \nimpl<T> CursorPtr<T> {\n    pub fn dangling() -> Self {\n        Self { at:\
    \ NonNull::dangling(), list: NonNull::dangling() }\n    }\n    pub fn next(&mut\
    \ self) -> Option<&mut Self> {\n        self.at = unsafe { self.at.as_ref() }.next_val.as_ref()?.0;\n\
    \        Some(self)\n    }\n    pub fn prev(&mut self) -> Option<&mut Self> {\n\
    \        self.at = unsafe { self.at.as_ref() }.prev?;\n        Some(self)\n  \
    \  }\n    pub fn advance(&mut self, by: isize) -> Option<&mut Self> {\n      \
    \  if by >= 0 {\n            for _ in 0..by {\n                self.next()?;\n\
    \            }\n        } else {\n            for _ in by..0 {\n             \
    \   self.prev()?;\n            }\n        }\n        Some(self)\n    }\n    ///\
    \ safety: must not alias list, prev, at, next\n    pub unsafe fn insert(&mut self,\
    \ val: T) -> Self {\n        let prev = self.at.as_ref().prev;\n        let new_node\
    \ =\n            self.list.as_mut().new_node(Node { prev, next_val: Some((self.at,\
    \ val)) });\n        self.at.as_mut().prev = Some(new_node);\n        if let Some(mut\
    \ prev) = prev {\n            prev.as_mut().next_val.as_mut().unwrap().0 = new_node;\n\
    \        } else {\n            self.list.as_mut().head = new_node;\n        }\n\
    \        self.at = new_node;\n        self.list.as_mut().len += 1;\n        let\
    \ next = new_node.as_ref().next_val.as_ref().map(|t| t.0);\n        assert_ne!(Some(new_node),\
    \ next);\n        Self { at: new_node, list: self.list }\n    }\n    /// safety:\
    \ must not alias list, prev, at, next\n    pub unsafe fn remove(&mut self) ->\
    \ Option<T> {\n        if self.at == self.list.as_mut().tail {\n            return\
    \ None;\n        }\n        let node = std::ptr::read(self.at.as_ptr());\n   \
    \     let (mut next, val) = node.next_val?;\n        if let Some(mut prev) = node.prev\
    \ {\n            *next.as_mut().prev.as_mut().unwrap() = prev;\n            prev.as_mut().next_val.as_mut().unwrap().0\
    \ = next;\n        } else {\n            next.as_mut().prev = None;\n        \
    \    self.list.as_mut().head = next;\n        }\n        self.at = next;\n   \
    \     self.list.as_mut().len -= 1;\n        Some(val)\n    }\n}\n"
  dependsOn:
  - src/draft/linked_list.rs
  isVerificationFile: false
  path: src/draft/linked_list/ptr.rs
  requiredBy: []
  timestamp: '2021-04-03 11:26:56+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/draft/linked_list/ptr.rs
layout: document
redirect_from:
- /library/src/draft/linked_list/ptr.rs
- /library/src/draft/linked_list/ptr.rs.html
title: src/draft/linked_list/ptr.rs
---
