use std::mem::ManuallyDrop;

pub fn transmute_vec<T, U>(v: Vec<T>) -> Vec<U> {
    let mut v = ManuallyDrop::new(v);
    unsafe { Vec::from_raw_parts(v.as_mut_ptr() as *mut _, v.len(), v.capacity()) }
}
