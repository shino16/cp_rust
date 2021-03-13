pub fn rle<T: Clone + Eq>(slice: &[T]) -> Vec<(T, usize)> {
    if let Some(v) = slice.get(0) {
        let mut now = v;
        let mut cnt = 1;
        let mut res = Vec::with_capacity(slice.len());
        for v in &slice[1..] {
            if now == v {
                cnt += 1;
            } else {
                res.push((now.clone(), cnt));
                now = v;
                cnt = 1;
            }
        }
        res.push((now.clone(), cnt));
        res
    } else {
        Vec::new()
    }
}
