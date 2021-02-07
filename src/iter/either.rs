pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<A, L, R> Iterator for Either<L, R>
where
    L: Iterator<Item = A>,
    R: Iterator<Item = A>,
{
    type Item = A;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Left(l) => l.next(),
            Self::Right(r) => r.next(),
        }
    }
}
