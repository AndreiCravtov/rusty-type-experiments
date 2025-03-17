use crate::family_pattern::{ConstructableTy2, TyConstructor2, TC2};
use std::marker::PhantomData;

// TODO: figure out how to represent type-level "flipping" of 2-constructor arguments
// TODO: figure out how to "apply" 2-TypeConstructors to obtain 1-TypeConstuctors, and so on
// TODO: with that figured out, expand typeclass hierarchy
//       - bi-functors, bi-applicatives, etc.,
//       - monad transformer encoding, how ??

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Flip<T: TyConstructor2>(PhantomData<T>);

#[repr(transparent)]
pub struct Flipped<T: ConstructableTy2>(
    pub TC2<T::Constructor, T::GenericParameter1, T::GenericParameter2>,
);

// impl<T: ConstructableTy2> ConstructableTy2 for Flipped<T> {
//     type GenericParameter1 = T::GenericParameter2;
//     type GenericParameter2 = T::GenericParameter1;
//     type Constructor = Flip<T::Constructor>;
// }
//
// impl<T: TyConstructor2> TyConstructor2 for Flip<T> {
//     type TC2<A, B> = Flipped<T::TC2<B, A>>;
// }
