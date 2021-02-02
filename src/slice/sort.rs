use std::mem::MaybeUninit;

// pub fn count_sort(a: &mut [usize], max: usize) {
// }

pub fn count_sort_by_key<T: Clone, F: FnMut(&T) -> usize>(
	a: &[T],
	max_key: usize,
	mut key: F,
) -> Vec<T> {
	let mut count = vec![0; max_key + 1];
	for e in a {
		count[key(e)] += 1;
	}
	for i in 0..max_key {
		count[i + 1] += count[i];
	}
	let mut res = Vec::with_capacity(a.len());
	{
        // this is safe
        // refer to https://docs.rs/uninit/0.4.0/uninit/extension_traits/trait.VecCapacity.html#method.get_backing_buffer_with_leaking_writes
		let res: &mut [MaybeUninit<T>] =
            unsafe { std::slice::from_raw_parts_mut(res.as_mut_ptr() as _, res.capacity()) };
        for e in a {
            count[key(e)] -= 1;
            res[count[key(e)]] = MaybeUninit::new(e.clone());
        }
    }
    unsafe {
        res.set_len(res.capacity());
    }
	res
}
