use crate::family_pattern::TyConstructor1;
use std::marker::PhantomData;

// TODO: figure out how to represent that something can "turn into" a type-constructor/type-constructable
//       of some typeclass, so that object is treated __AS__ that typeclass
//       e.g.
//         - say we have a newtype wrapper `struct CloneVec<T>(Vec<T>, CloneWit<T>)` which contains
//           a type-witness that `T: Clone`
//         - normal `Vec<T>` cannot be an `Applicative`, but `CloneVec<T>` can be an `Applicative`
//           since `T: Clone` is satisfied by the type-witness. So it is perfectly possible to define
//           `impl<T: Clone> From<Vec<T>> for CloneVec<T>`..., thus we would say that for some types
//           `T,V` if `V: Into<CloneVec<T>>` then `Into1<V>` should ALSO implement `Applicative`...
//           and this boundary can be smoothed over even more with `Some<dyn Trait>` containers (possibly!!)
//           such that IF a `Vec<T>` is such that it is `Clone` then I can pass it into items which expect
//           `Applicative` AND IT WILL AUTOWRAP-UNWRAP and just work with minimal friction

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Into1<T: TyConstructor1>(PhantomData<T>);
