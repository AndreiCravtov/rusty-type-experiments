pub trait Semigroup {
    #[must_use]
    fn scombine(self, other: Self) -> Self;
}
