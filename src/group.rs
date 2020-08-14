pub trait Group {
    fn identity() -> Self;
    fn inverse(&self) -> Self;
    fn compose(&self, other: &Self) -> Self;
}
