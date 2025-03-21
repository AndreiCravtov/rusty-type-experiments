//! Ownership-related utility traits
//!

/// An alternative to [`AsRef`] that does not force the reference type to be a pointer itself.
///
/// This lets us create implementations for recursive traits that take the resulting
/// [`Self::Output`] reference type, useful for generic contexts.
///
/// Also see [`ToMut`].
pub trait ToRef<'a> {
    type Output;

    fn to_ref(&'a self) -> Self::Output;
}

/// An alternative to [`AsMut`] that does not force the reference type to be a pointer itself.
///
/// This lets us create implementations for recursive traits that take the resulting
/// [`Self::Output`] reference type, useful for generic contexts.
///
/// Also see [`ToRef`].
pub trait ToMut<'a> {
    type Output;

    fn to_mut(&'a mut self) -> Self::Output;
}
