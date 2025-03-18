//! Types used for indexing into datastructures like `HList`s, `HBinaryTree`s, coproducts, etc.

use core::marker::PhantomData;

/// Used as an index into an `HList`.
///
/// `Here` is 0, pointing to the head of the HList.
///
/// Users should normally allow type inference to create this type.
pub struct Here {
    _priv: (),
}

/// Used as an index into an `HList`.
///
/// `There<T>` is 1 + `T`.
///
/// Users should normally allow type inference to create this type.
pub struct There<T> {
    _marker: PhantomData<T>,
}

/// An index denoting that `Suffix` is just that.
pub struct Suffixed<Suffix> {
    _marker: PhantomData<Suffix>,
}

// /// Index for the case where we don't need to do any transmogrifying at all because the source
// /// type is the same as the target type.
// pub enum IdentityTransMog {}
//
// /// Index for the case where we need to do work in order to transmogrify one type into another.
// pub struct DoTransmog<PluckByKeyIndex, TransMogIndex> {
//     _marker1: PhantomData<PluckByKeyIndex>,
//     _marker2: PhantomData<TransMogIndex>,
// }
//
// /// Index type wrapper for transmogrifying a generic Source to a generic Target
// pub struct LabelledGenericTransmogIndicesWrapper<T>(PhantomData<T>);
//
// /// Index type wrapper for transmogrifying a generic plucked Source to a generic Target
// pub struct PluckedLabelledGenericIndicesWrapper<T>(T);
//
// /// Index type wrapper for transmogrifying through a (known) container (e.g. `Vec`).
// pub struct MappingIndicesWrapper<T>(PhantomData<T>);
