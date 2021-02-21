use std::mem::MaybeUninit;

pub fn count_sort_bytes(slice: &[u8], out: &mut Vec<u8>, max_key: usize) {
    count_sort(slice, out, max_key, |x| x as usize);
}

pub fn count_sort<T: Copy, F: FnMut(T) -> usize>(
    slice: &[T],
    out: &mut Vec<T>,
    max_key: usize,
    mut key: F,
) {
    let mut count = vec![0; max_key + 1];
    for &e in slice {
        count[key(e)] += 1;
    }
    for i in 0..max_key {
        count[i + 1] += count[i];
    }
    out.clear();
    out.reserve(slice.len());
    {
        // this is safe
        // refer to https://docs.rs/uninit/0.4.0/uninit/extension_traits/trait.VecCapacity.html#method.get_backing_buffer_with_leaking_writes
        let out: &mut [MaybeUninit<T>] =
            unsafe { std::slice::from_raw_parts_mut(out.as_mut_ptr() as _, slice.len()) };
        for &e in slice.iter().rev() {
            count[key(e)] -= 1;
            out[count[key(e)]] = MaybeUninit::new(e.clone());
        }
    }
    unsafe {
        out.set_len(slice.len());
    }
}
