// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(unstable_name_collisions)]
#![allow(dead_code)]

use core::cmp;
use core::fmt;
use core::mem;
use core::ptr::{self, NonNull};
use core::usize;

pub use core::alloc::{Layout, LayoutErr};

fn new_layout_err() -> LayoutErr {
	Layout::from_size_align(1, 3).unwrap_err()
}

pub fn handle_alloc_error(layout: Layout) -> ! {
	panic!("encountered allocation error: {:?}", layout)
}

pub trait UnstableLayoutMethods {
	fn padding_needed_for(&self, align: usize) -> usize;
	fn repeat(&self, n: usize) -> Result<(Layout, usize), LayoutErr>;
	fn array<T>(n: usize) -> Result<Layout, LayoutErr>;
}

impl UnstableLayoutMethods for Layout {
	fn padding_needed_for(&self, align: usize) -> usize {
		let len = self.size();


		let len_rounded_up = len.wrapping_add(align).wrapping_sub(1) & !align.wrapping_sub(1);
		len_rounded_up.wrapping_sub(len)
	}

	fn repeat(&self, n: usize) -> Result<(Layout, usize), LayoutErr> {
		let padded_size = self
			.size()
			.checked_add(self.padding_needed_for(self.align()))
			.ok_or_else(new_layout_err)?;
		let alloc_size = padded_size.checked_mul(n).ok_or_else(new_layout_err)?;

		unsafe {
			Ok((
				Layout::from_size_align_unchecked(alloc_size, self.align()),
				padded_size,
			))
		}
	}

	fn array<T>(n: usize) -> Result<Layout, LayoutErr> {
		Layout::new::<T>().repeat(n).map(|(k, offs)| {
			debug_assert!(offs == mem::size_of::<T>());
			k
		})
	}
}

#[derive(Debug)]
pub struct Excess(pub NonNull<u8>, pub usize);

fn size_align<T>() -> (usize, usize) {
	(mem::size_of::<T>(), mem::align_of::<T>())
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AllocErr;

impl fmt::Display for AllocErr {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str("memory allocation failed")
	}
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CannotReallocInPlace;

impl CannotReallocInPlace {
	pub fn description(&self) -> &str {
		"cannot reallocate allocator's memory in place"
	}
}

impl fmt::Display for CannotReallocInPlace {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.description())
	}
}

pub unsafe trait Alloc {
	unsafe fn alloc(&mut self, layout: Layout) -> Result<NonNull<u8>, AllocErr>;

	unsafe fn dealloc(&mut self, ptr: NonNull<u8>, layout: Layout);


	#[inline]
	fn usable_size(&self, layout: &Layout) -> (usize, usize) {
		(layout.size(), layout.size())
	}


	unsafe fn realloc(
		&mut self,
		ptr: NonNull<u8>,
		layout: Layout,
		new_size: usize,
	) -> Result<NonNull<u8>, AllocErr> {
		let old_size = layout.size();

		if new_size >= old_size {
			if let Ok(()) = self.grow_in_place(ptr, layout, new_size) {
				return Ok(ptr);
			}
		} else if new_size < old_size {
			if let Ok(()) = self.shrink_in_place(ptr, layout, new_size) {
				return Ok(ptr);
			}
		}

		let new_layout = Layout::from_size_align_unchecked(new_size, layout.align());
		let result = self.alloc(new_layout);
		if let Ok(new_ptr) = result {
			ptr::copy_nonoverlapping(
				ptr.as_ptr(),
				new_ptr.as_ptr(),
				cmp::min(old_size, new_size),
			);
			self.dealloc(ptr, layout);
		}
		result
	}

	unsafe fn alloc_zeroed(&mut self, layout: Layout) -> Result<NonNull<u8>, AllocErr> {
		let size = layout.size();
		let p = self.alloc(layout);
		if let Ok(p) = p {
			ptr::write_bytes(p.as_ptr(), 0, size);
		}
		p
	}

	unsafe fn alloc_excess(&mut self, layout: Layout) -> Result<Excess, AllocErr> {
		let usable_size = self.usable_size(&layout);
		self.alloc(layout).map(|p| Excess(p, usable_size.1))
	}

	unsafe fn realloc_excess(
		&mut self,
		ptr: NonNull<u8>,
		layout: Layout,
		new_size: usize,
	) -> Result<Excess, AllocErr> {
		let new_layout = Layout::from_size_align_unchecked(new_size, layout.align());
		let usable_size = self.usable_size(&new_layout);
		self.realloc(ptr, layout, new_size).map(|p| Excess(p, usable_size.1))
	}

	unsafe fn grow_in_place(
		&mut self,
		_ptr: NonNull<u8>,
		layout: Layout,
		new_size: usize,
	) -> Result<(), CannotReallocInPlace> {
		debug_assert!(new_size >= layout.size());
		let (_l, u) = self.usable_size(&layout);
		if new_size <= u {
			Ok(())
		} else {
			Err(CannotReallocInPlace)
		}
	}

	unsafe fn shrink_in_place(
		&mut self,
		_ptr: NonNull<u8>,
		layout: Layout,
		new_size: usize,
	) -> Result<(), CannotReallocInPlace> {
		debug_assert!(new_size <= layout.size());
		let (l, _u) = self.usable_size(&layout);
		if l <= new_size {
			Ok(())
		} else {
			Err(CannotReallocInPlace)
		}
	}


	fn alloc_one<T>(&mut self) -> Result<NonNull<T>, AllocErr>
	where
		Self: Sized,
	{
		let k = Layout::new::<T>();
		if k.size() > 0 {
			unsafe { self.alloc(k).map(|p| p.cast()) }
		} else {
			Err(AllocErr)
		}
	}

	unsafe fn dealloc_one<T>(&mut self, ptr: NonNull<T>)
	where
		Self: Sized,
	{
		let k = Layout::new::<T>();
		if k.size() > 0 {
			self.dealloc(ptr.cast(), k);
		}
	}

	fn alloc_array<T>(&mut self, n: usize) -> Result<NonNull<T>, AllocErr>
	where
		Self: Sized,
	{
		match Layout::array::<T>(n) {
			Ok(layout) if layout.size() > 0 => unsafe {
				self.alloc(layout).map(|p| p.cast())
			},
			_ => Err(AllocErr),
		}
	}

	unsafe fn realloc_array<T>(
		&mut self,
		ptr: NonNull<T>,
		n_old: usize,
		n_new: usize,
	) -> Result<NonNull<T>, AllocErr>
	where
		Self: Sized,
	{
		match (Layout::array::<T>(n_old), Layout::array::<T>(n_new)) {
			(Ok(ref k_old), Ok(ref k_new)) if k_old.size() > 0 && k_new.size() > 0 => {
				debug_assert!(k_old.align() == k_new.align());
				self.realloc(ptr.cast(), k_old.clone(), k_new.size()).map(NonNull::cast)
			},
			_ => Err(AllocErr),
		}
	}

	unsafe fn dealloc_array<T>(&mut self, ptr: NonNull<T>, n: usize) -> Result<(), AllocErr>
	where
		Self: Sized,
	{
		match Layout::array::<T>(n) {
			Ok(k) if k.size() > 0 => {
				self.dealloc(ptr.cast(), k);
				Ok(())
			},
			_ => Err(AllocErr),
		}
	}
}
