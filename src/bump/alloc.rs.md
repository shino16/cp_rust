---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links:
    - http://opensource.org/licenses/MIT>,
    - http://rust-lang.org/COPYRIGHT.
    - http://www.apache.org/licenses/LICENSE-2.0>
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// Copyright 2015 The Rust Project Developers. See the COPYRIGHT\n// file\
    \ at the top-level directory of this distribution and at\n// http://rust-lang.org/COPYRIGHT.\n\
    //\n// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or\n// http://www.apache.org/licenses/LICENSE-2.0>\
    \ or the MIT license\n// <LICENSE-MIT or http://opensource.org/licenses/MIT>,\
    \ at your\n// option. This file may not be copied, modified, or distributed\n\
    // except according to those terms.\n\n#![allow(unstable_name_collisions)]\n#![allow(dead_code)]\n\
    \nuse core::cmp;\nuse core::fmt;\nuse core::mem;\nuse core::ptr::{self, NonNull};\n\
    use core::usize;\n\npub use core::alloc::{Layout, LayoutErr};\n\nfn new_layout_err()\
    \ -> LayoutErr {\n\tLayout::from_size_align(1, 3).unwrap_err()\n}\n\npub fn handle_alloc_error(layout:\
    \ Layout) -> ! {\n\tpanic!(\"encountered allocation error: {:?}\", layout)\n}\n\
    \npub trait UnstableLayoutMethods {\n\tfn padding_needed_for(&self, align: usize)\
    \ -> usize;\n\tfn repeat(&self, n: usize) -> Result<(Layout, usize), LayoutErr>;\n\
    \tfn array<T>(n: usize) -> Result<Layout, LayoutErr>;\n}\n\nimpl UnstableLayoutMethods\
    \ for Layout {\n\tfn padding_needed_for(&self, align: usize) -> usize {\n\t\t\
    let len = self.size();\n\n\n\t\tlet len_rounded_up = len.wrapping_add(align).wrapping_sub(1)\
    \ & !align.wrapping_sub(1);\n\t\tlen_rounded_up.wrapping_sub(len)\n\t}\n\n\tfn\
    \ repeat(&self, n: usize) -> Result<(Layout, usize), LayoutErr> {\n\t\tlet padded_size\
    \ = self\n\t\t\t.size()\n\t\t\t.checked_add(self.padding_needed_for(self.align()))\n\
    \t\t\t.ok_or_else(new_layout_err)?;\n\t\tlet alloc_size = padded_size.checked_mul(n).ok_or_else(new_layout_err)?;\n\
    \n\t\tunsafe {\n\t\t\tOk((\n\t\t\t\tLayout::from_size_align_unchecked(alloc_size,\
    \ self.align()),\n\t\t\t\tpadded_size,\n\t\t\t))\n\t\t}\n\t}\n\n\tfn array<T>(n:\
    \ usize) -> Result<Layout, LayoutErr> {\n\t\tLayout::new::<T>().repeat(n).map(|(k,\
    \ offs)| {\n\t\t\tdebug_assert!(offs == mem::size_of::<T>());\n\t\t\tk\n\t\t})\n\
    \t}\n}\n\n#[derive(Debug)]\npub struct Excess(pub NonNull<u8>, pub usize);\n\n\
    fn size_align<T>() -> (usize, usize) {\n\t(mem::size_of::<T>(), mem::align_of::<T>())\n\
    }\n\n#[derive(Clone, PartialEq, Eq, Debug)]\npub struct AllocErr;\n\nimpl fmt::Display\
    \ for AllocErr {\n\tfn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {\n\t\
    \tf.write_str(\"memory allocation failed\")\n\t}\n}\n\n#[derive(Clone, PartialEq,\
    \ Eq, Debug)]\npub struct CannotReallocInPlace;\n\nimpl CannotReallocInPlace {\n\
    \tpub fn description(&self) -> &str {\n\t\t\"cannot reallocate allocator's memory\
    \ in place\"\n\t}\n}\n\nimpl fmt::Display for CannotReallocInPlace {\n\tfn fmt(&self,\
    \ f: &mut fmt::Formatter) -> fmt::Result {\n\t\twrite!(f, \"{}\", self.description())\n\
    \t}\n}\n\npub unsafe trait Alloc {\n\tunsafe fn alloc(&mut self, layout: Layout)\
    \ -> Result<NonNull<u8>, AllocErr>;\n\n\tunsafe fn dealloc(&mut self, ptr: NonNull<u8>,\
    \ layout: Layout);\n\n\n\t#[inline]\n\tfn usable_size(&self, layout: &Layout)\
    \ -> (usize, usize) {\n\t\t(layout.size(), layout.size())\n\t}\n\n\n\tunsafe fn\
    \ realloc(\n\t\t&mut self,\n\t\tptr: NonNull<u8>,\n\t\tlayout: Layout,\n\t\tnew_size:\
    \ usize,\n\t) -> Result<NonNull<u8>, AllocErr> {\n\t\tlet old_size = layout.size();\n\
    \n\t\tif new_size >= old_size {\n\t\t\tif let Ok(()) = self.grow_in_place(ptr,\
    \ layout, new_size) {\n\t\t\t\treturn Ok(ptr);\n\t\t\t}\n\t\t} else if new_size\
    \ < old_size {\n\t\t\tif let Ok(()) = self.shrink_in_place(ptr, layout, new_size)\
    \ {\n\t\t\t\treturn Ok(ptr);\n\t\t\t}\n\t\t}\n\n\t\tlet new_layout = Layout::from_size_align_unchecked(new_size,\
    \ layout.align());\n\t\tlet result = self.alloc(new_layout);\n\t\tif let Ok(new_ptr)\
    \ = result {\n\t\t\tptr::copy_nonoverlapping(\n\t\t\t\tptr.as_ptr(),\n\t\t\t\t\
    new_ptr.as_ptr(),\n\t\t\t\tcmp::min(old_size, new_size),\n\t\t\t);\n\t\t\tself.dealloc(ptr,\
    \ layout);\n\t\t}\n\t\tresult\n\t}\n\n\tunsafe fn alloc_zeroed(&mut self, layout:\
    \ Layout) -> Result<NonNull<u8>, AllocErr> {\n\t\tlet size = layout.size();\n\t\
    \tlet p = self.alloc(layout);\n\t\tif let Ok(p) = p {\n\t\t\tptr::write_bytes(p.as_ptr(),\
    \ 0, size);\n\t\t}\n\t\tp\n\t}\n\n\tunsafe fn alloc_excess(&mut self, layout:\
    \ Layout) -> Result<Excess, AllocErr> {\n\t\tlet usable_size = self.usable_size(&layout);\n\
    \t\tself.alloc(layout).map(|p| Excess(p, usable_size.1))\n\t}\n\n\tunsafe fn realloc_excess(\n\
    \t\t&mut self,\n\t\tptr: NonNull<u8>,\n\t\tlayout: Layout,\n\t\tnew_size: usize,\n\
    \t) -> Result<Excess, AllocErr> {\n\t\tlet new_layout = Layout::from_size_align_unchecked(new_size,\
    \ layout.align());\n\t\tlet usable_size = self.usable_size(&new_layout);\n\t\t\
    self.realloc(ptr, layout, new_size).map(|p| Excess(p, usable_size.1))\n\t}\n\n\
    \tunsafe fn grow_in_place(\n\t\t&mut self,\n\t\t_ptr: NonNull<u8>,\n\t\tlayout:\
    \ Layout,\n\t\tnew_size: usize,\n\t) -> Result<(), CannotReallocInPlace> {\n\t\
    \tdebug_assert!(new_size >= layout.size());\n\t\tlet (_l, u) = self.usable_size(&layout);\n\
    \t\tif new_size <= u {\n\t\t\tOk(())\n\t\t} else {\n\t\t\tErr(CannotReallocInPlace)\n\
    \t\t}\n\t}\n\n\tunsafe fn shrink_in_place(\n\t\t&mut self,\n\t\t_ptr: NonNull<u8>,\n\
    \t\tlayout: Layout,\n\t\tnew_size: usize,\n\t) -> Result<(), CannotReallocInPlace>\
    \ {\n\t\tdebug_assert!(new_size <= layout.size());\n\t\tlet (l, _u) = self.usable_size(&layout);\n\
    \t\tif l <= new_size {\n\t\t\tOk(())\n\t\t} else {\n\t\t\tErr(CannotReallocInPlace)\n\
    \t\t}\n\t}\n\n\n\tfn alloc_one<T>(&mut self) -> Result<NonNull<T>, AllocErr>\n\
    \twhere\n\t\tSelf: Sized,\n\t{\n\t\tlet k = Layout::new::<T>();\n\t\tif k.size()\
    \ > 0 {\n\t\t\tunsafe { self.alloc(k).map(|p| p.cast()) }\n\t\t} else {\n\t\t\t\
    Err(AllocErr)\n\t\t}\n\t}\n\n\tunsafe fn dealloc_one<T>(&mut self, ptr: NonNull<T>)\n\
    \twhere\n\t\tSelf: Sized,\n\t{\n\t\tlet k = Layout::new::<T>();\n\t\tif k.size()\
    \ > 0 {\n\t\t\tself.dealloc(ptr.cast(), k);\n\t\t}\n\t}\n\n\tfn alloc_array<T>(&mut\
    \ self, n: usize) -> Result<NonNull<T>, AllocErr>\n\twhere\n\t\tSelf: Sized,\n\
    \t{\n\t\tmatch Layout::array::<T>(n) {\n\t\t\tOk(layout) if layout.size() > 0\
    \ => unsafe {\n\t\t\t\tself.alloc(layout).map(|p| p.cast())\n\t\t\t},\n\t\t\t\
    _ => Err(AllocErr),\n\t\t}\n\t}\n\n\tunsafe fn realloc_array<T>(\n\t\t&mut self,\n\
    \t\tptr: NonNull<T>,\n\t\tn_old: usize,\n\t\tn_new: usize,\n\t) -> Result<NonNull<T>,\
    \ AllocErr>\n\twhere\n\t\tSelf: Sized,\n\t{\n\t\tmatch (Layout::array::<T>(n_old),\
    \ Layout::array::<T>(n_new)) {\n\t\t\t(Ok(ref k_old), Ok(ref k_new)) if k_old.size()\
    \ > 0 && k_new.size() > 0 => {\n\t\t\t\tdebug_assert!(k_old.align() == k_new.align());\n\
    \t\t\t\tself.realloc(ptr.cast(), k_old.clone(), k_new.size()).map(NonNull::cast)\n\
    \t\t\t},\n\t\t\t_ => Err(AllocErr),\n\t\t}\n\t}\n\n\tunsafe fn dealloc_array<T>(&mut\
    \ self, ptr: NonNull<T>, n: usize) -> Result<(), AllocErr>\n\twhere\n\t\tSelf:\
    \ Sized,\n\t{\n\t\tmatch Layout::array::<T>(n) {\n\t\t\tOk(k) if k.size() > 0\
    \ => {\n\t\t\t\tself.dealloc(ptr.cast(), k);\n\t\t\t\tOk(())\n\t\t\t},\n\t\t\t\
    _ => Err(AllocErr),\n\t\t}\n\t}\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/bump/alloc.rs
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/bump/alloc.rs
layout: document
redirect_from:
- /library/src/bump/alloc.rs
- /library/src/bump/alloc.rs.html
title: src/bump/alloc.rs
---
