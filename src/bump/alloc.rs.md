---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "// Copyright 2015 The Rust Project Developers. See the COPYRIGHT\n// file\
    \ at the top-level directory of this distribution and at\n// http://rust-lang.org/COPYRIGHT.\n\
    //\n// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or\n// http://www.apache.org/licenses/LICENSE-2.0>\
    \ or the MIT license\n// <LICENSE-MIT or http://opensource.org/licenses/MIT>,\
    \ at your\n// option. This file may not be copied, modified, or distributed\n\
    // except according to those terms.\n\n#![allow(unstable_name_collisions)]\n#![allow(dead_code)]\n\
    \nuse core::cmp;\nuse core::fmt;\nuse core::mem;\nuse core::ptr::{self, NonNull};\n\
    use core::usize;\n\npub use core::alloc::{Layout, LayoutErr};\n\nfn new_layout_err()\
    \ -> LayoutErr {\n    Layout::from_size_align(1, 3).unwrap_err()\n}\n\npub fn\
    \ handle_alloc_error(layout: Layout) -> ! {\n    panic!(\"encountered allocation\
    \ error: {:?}\", layout)\n}\n\npub trait UnstableLayoutMethods {\n    fn padding_needed_for(&self,\
    \ align: usize) -> usize;\n    fn repeat(&self, n: usize) -> Result<(Layout, usize),\
    \ LayoutErr>;\n    fn array<T>(n: usize) -> Result<Layout, LayoutErr>;\n}\n\n\
    impl UnstableLayoutMethods for Layout {\n    fn padding_needed_for(&self, align:\
    \ usize) -> usize {\n        let len = self.size();\n\n\n        let len_rounded_up\
    \ = len.wrapping_add(align).wrapping_sub(1) & !align.wrapping_sub(1);\n      \
    \  len_rounded_up.wrapping_sub(len)\n    }\n\n    fn repeat(&self, n: usize) ->\
    \ Result<(Layout, usize), LayoutErr> {\n        let padded_size = self\n     \
    \       .size()\n            .checked_add(self.padding_needed_for(self.align()))\n\
    \            .ok_or_else(new_layout_err)?;\n        let alloc_size = padded_size.checked_mul(n).ok_or_else(new_layout_err)?;\n\
    \n        unsafe {\n            Ok((\n                Layout::from_size_align_unchecked(alloc_size,\
    \ self.align()),\n                padded_size,\n            ))\n        }\n  \
    \  }\n\n    fn array<T>(n: usize) -> Result<Layout, LayoutErr> {\n        Layout::new::<T>().repeat(n).map(|(k,\
    \ offs)| {\n            debug_assert!(offs == mem::size_of::<T>());\n        \
    \    k\n        })\n    }\n}\n\n#[derive(Debug)]\npub struct Excess(pub NonNull<u8>,\
    \ pub usize);\n\nfn size_align<T>() -> (usize, usize) {\n    (mem::size_of::<T>(),\
    \ mem::align_of::<T>())\n}\n\n#[derive(Clone, PartialEq, Eq, Debug)]\npub struct\
    \ AllocErr;\n\nimpl fmt::Display for AllocErr {\n    fn fmt(&self, f: &mut fmt::Formatter)\
    \ -> fmt::Result {\n        f.write_str(\"memory allocation failed\")\n    }\n\
    }\n\n#[derive(Clone, PartialEq, Eq, Debug)]\npub struct CannotReallocInPlace;\n\
    \nimpl CannotReallocInPlace {\n    pub fn description(&self) -> &str {\n     \
    \   \"cannot reallocate allocator's memory in place\"\n    }\n}\n\nimpl fmt::Display\
    \ for CannotReallocInPlace {\n    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result\
    \ {\n        write!(f, \"{}\", self.description())\n    }\n}\n\npub unsafe trait\
    \ Alloc {\n    unsafe fn alloc(&mut self, layout: Layout) -> Result<NonNull<u8>,\
    \ AllocErr>;\n\n    unsafe fn dealloc(&mut self, ptr: NonNull<u8>, layout: Layout);\n\
    \n\n    #[inline]\n    fn usable_size(&self, layout: &Layout) -> (usize, usize)\
    \ {\n        (layout.size(), layout.size())\n    }\n\n\n    unsafe fn realloc(\n\
    \        &mut self,\n        ptr: NonNull<u8>,\n        layout: Layout,\n    \
    \    new_size: usize,\n    ) -> Result<NonNull<u8>, AllocErr> {\n        let old_size\
    \ = layout.size();\n\n        if new_size >= old_size {\n            if let Ok(())\
    \ = self.grow_in_place(ptr, layout, new_size) {\n                return Ok(ptr);\n\
    \            }\n        } else if new_size < old_size {\n            if let Ok(())\
    \ = self.shrink_in_place(ptr, layout, new_size) {\n                return Ok(ptr);\n\
    \            }\n        }\n\n        let new_layout = Layout::from_size_align_unchecked(new_size,\
    \ layout.align());\n        let result = self.alloc(new_layout);\n        if let\
    \ Ok(new_ptr) = result {\n            ptr::copy_nonoverlapping(\n            \
    \    ptr.as_ptr(),\n                new_ptr.as_ptr(),\n                cmp::min(old_size,\
    \ new_size),\n            );\n            self.dealloc(ptr, layout);\n       \
    \ }\n        result\n    }\n\n    unsafe fn alloc_zeroed(&mut self, layout: Layout)\
    \ -> Result<NonNull<u8>, AllocErr> {\n        let size = layout.size();\n    \
    \    let p = self.alloc(layout);\n        if let Ok(p) = p {\n            ptr::write_bytes(p.as_ptr(),\
    \ 0, size);\n        }\n        p\n    }\n\n    unsafe fn alloc_excess(&mut self,\
    \ layout: Layout) -> Result<Excess, AllocErr> {\n        let usable_size = self.usable_size(&layout);\n\
    \        self.alloc(layout).map(|p| Excess(p, usable_size.1))\n    }\n\n    unsafe\
    \ fn realloc_excess(\n        &mut self,\n        ptr: NonNull<u8>,\n        layout:\
    \ Layout,\n        new_size: usize,\n    ) -> Result<Excess, AllocErr> {\n   \
    \     let new_layout = Layout::from_size_align_unchecked(new_size, layout.align());\n\
    \        let usable_size = self.usable_size(&new_layout);\n        self.realloc(ptr,\
    \ layout, new_size).map(|p| Excess(p, usable_size.1))\n    }\n\n    unsafe fn\
    \ grow_in_place(\n        &mut self,\n        _ptr: NonNull<u8>,\n        layout:\
    \ Layout,\n        new_size: usize,\n    ) -> Result<(), CannotReallocInPlace>\
    \ {\n        debug_assert!(new_size >= layout.size());\n        let (_l, u) =\
    \ self.usable_size(&layout);\n        if new_size <= u {\n            Ok(())\n\
    \        } else {\n            Err(CannotReallocInPlace)\n        }\n    }\n\n\
    \    unsafe fn shrink_in_place(\n        &mut self,\n        _ptr: NonNull<u8>,\n\
    \        layout: Layout,\n        new_size: usize,\n    ) -> Result<(), CannotReallocInPlace>\
    \ {\n        debug_assert!(new_size <= layout.size());\n        let (l, _u) =\
    \ self.usable_size(&layout);\n        if l <= new_size {\n            Ok(())\n\
    \        } else {\n            Err(CannotReallocInPlace)\n        }\n    }\n\n\
    \n    fn alloc_one<T>(&mut self) -> Result<NonNull<T>, AllocErr>\n    where\n\
    \        Self: Sized,\n    {\n        let k = Layout::new::<T>();\n        if\
    \ k.size() > 0 {\n            unsafe { self.alloc(k).map(|p| p.cast()) }\n   \
    \     } else {\n            Err(AllocErr)\n        }\n    }\n\n    unsafe fn dealloc_one<T>(&mut\
    \ self, ptr: NonNull<T>)\n    where\n        Self: Sized,\n    {\n        let\
    \ k = Layout::new::<T>();\n        if k.size() > 0 {\n            self.dealloc(ptr.cast(),\
    \ k);\n        }\n    }\n\n    fn alloc_array<T>(&mut self, n: usize) -> Result<NonNull<T>,\
    \ AllocErr>\n    where\n        Self: Sized,\n    {\n        match Layout::array::<T>(n)\
    \ {\n            Ok(layout) if layout.size() > 0 => unsafe {\n               \
    \ self.alloc(layout).map(|p| p.cast())\n            },\n            _ => Err(AllocErr),\n\
    \        }\n    }\n\n    unsafe fn realloc_array<T>(\n        &mut self,\n   \
    \     ptr: NonNull<T>,\n        n_old: usize,\n        n_new: usize,\n    ) ->\
    \ Result<NonNull<T>, AllocErr>\n    where\n        Self: Sized,\n    {\n     \
    \   match (Layout::array::<T>(n_old), Layout::array::<T>(n_new)) {\n         \
    \   (Ok(ref k_old), Ok(ref k_new)) if k_old.size() > 0 && k_new.size() > 0 =>\
    \ {\n                debug_assert!(k_old.align() == k_new.align());\n        \
    \        self.realloc(ptr.cast(), k_old.clone(), k_new.size()).map(NonNull::cast)\n\
    \            },\n            _ => Err(AllocErr),\n        }\n    }\n\n    unsafe\
    \ fn dealloc_array<T>(&mut self, ptr: NonNull<T>, n: usize) -> Result<(), AllocErr>\n\
    \    where\n        Self: Sized,\n    {\n        match Layout::array::<T>(n) {\n\
    \            Ok(k) if k.size() > 0 => {\n                self.dealloc(ptr.cast(),\
    \ k);\n                Ok(())\n            },\n            _ => Err(AllocErr),\n\
    \        }\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/bump/alloc.rs
  requiredBy: []
  timestamp: '2020-11-24 01:55:32+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/bump/alloc.rs
layout: document
redirect_from:
- /library/src/bump/alloc.rs
- /library/src/bump/alloc.rs.html
title: src/bump/alloc.rs
---
