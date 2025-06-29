pub struct Container<T> {
    pub value: T,
}

pub enum Either<L, R> {
    Left(L),
    Right(R),
}

pub struct Graph<T> {
    pub vertices: Vec<T>,
    pub edges: Vec<(T, T)>,
}