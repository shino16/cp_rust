// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::rand::seed;
use std::collections::{HashMap, HashSet};
use std::default::Default;
use std::hash::{BuildHasherDefault, Hash, Hasher};
use std::ops::BitXor;

pub type FxBuildHasher = BuildHasherDefault<FxHasher>;

pub type FxHashMap<K, V> = HashMap<K, V, FxBuildHasher>;

pub type FxHashSet<V> = HashSet<V, FxBuildHasher>;

const ROTATE: u32 = 5;
static mut SEED64: u64 = 0;
static mut SEED32: u32 = 0;
static mut SEED: usize = 0;

#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XCU")]
static INIT: unsafe extern "C" fn() = {
	#[cfg_attr(target_os = "linux", link_section = ".text.startup")]
	unsafe extern "C" fn init() {
		SEED64 = seed::seed64();
		SEED32 = SEED64 as u32;
		SEED = SEED64 as usize;
	}
	init
};

trait HashWord {
	fn hash_word(&mut self, word: Self);
}

macro_rules! impl_hash_word {
	($($ty:ty = $key:ident),* $(,)*) => { $(
		impl HashWord for $ty {
			fn hash_word(&mut self, word: Self) {
				*self = self.rotate_left(ROTATE).bitxor(word).wrapping_mul(unsafe { $key });
			}
		}
	)* }
}

impl_hash_word!(usize = SEED, u32 = SEED32, u64 = SEED64);

fn read_u32(bytes: &[u8]) -> u32 {
	let mut data = 0;
	unsafe {
		std::ptr::copy_nonoverlapping(bytes.as_ptr(), &mut data as *mut _ as *mut u8, 4);
	}
	data
}

fn read_u64(bytes: &[u8]) -> u64 {
	let mut data = 0;
	unsafe {
		std::ptr::copy_nonoverlapping(bytes.as_ptr(), &mut data as *mut _ as *mut u8, 8);
	}
	data
}

#[allow(dead_code)]
fn write32(mut hash: u32, mut bytes: &[u8]) -> u32 {
	while bytes.len() >= 4 {
		let n = read_u32(bytes);
		hash.hash_word(n);
		bytes = bytes.split_at(4).1;
	}

	for byte in bytes {
		hash.hash_word(*byte as u32);
	}
	hash
}

#[allow(dead_code)]
fn write64(mut hash: u64, mut bytes: &[u8]) -> u64 {
	while bytes.len() >= 8 {
		let n = read_u64(bytes);
		hash.hash_word(n);
		bytes = bytes.split_at(8).1;
	}

	if bytes.len() >= 4 {
		let n = read_u32(bytes);
		hash.hash_word(n as u64);
		bytes = bytes.split_at(4).1;
	}

	for byte in bytes {
		hash.hash_word(*byte as u64);
	}
	hash
}

#[cfg(target_pointer_width = "32")]
fn write(hash: usize, bytes: &[u8]) -> usize {
	write32(hash as u32, bytes) as usize
}

#[cfg(target_pointer_width = "64")]
fn write(hash: usize, bytes: &[u8]) -> usize {
	write64(hash as u64, bytes) as usize
}

#[derive(Debug, Clone)]
pub struct FxHasher {
	hash: usize,
}

impl Default for FxHasher {
	fn default() -> FxHasher {
		FxHasher { hash: 0 }
	}
}

impl Hasher for FxHasher {
	fn write(&mut self, bytes: &[u8]) {
		self.hash = write(self.hash, bytes);
	}

	fn write_u8(&mut self, i: u8) {
		self.hash.hash_word(i as usize);
	}

	fn write_u16(&mut self, i: u16) {
		self.hash.hash_word(i as usize);
	}

	fn write_u32(&mut self, i: u32) {
		self.hash.hash_word(i as usize);
	}

	#[cfg(target_pointer_width = "32")]
	fn write_u64(&mut self, i: u64) {
		self.hash.hash_word(i as usize);
		self.hash.hash_word((i >> 32) as usize);
	}

	#[cfg(target_pointer_width = "64")]
	fn write_u64(&mut self, i: u64) {
		self.hash.hash_word(i as usize);
	}

	fn write_usize(&mut self, i: usize) {
		self.hash.hash_word(i);
	}

	fn finish(&self) -> u64 {
		self.hash as u64
	}
}

pub fn hash<T: Hash + ?Sized>(v: &T) -> usize {
	let mut state = FxHasher::default();
	v.hash(&mut state);
	state.finish() as usize
}
