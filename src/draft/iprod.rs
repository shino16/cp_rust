macro_rules! iprod {
    ($head:expr) => {
        $head.into_iter()
    };
    ($head:expr, $($tail:expr),*) => (
        $head.into_iter().flat_map(|e| {
            std::iter::repeat(e).zip(iprod!($($tail),*))
        })
    );
}
