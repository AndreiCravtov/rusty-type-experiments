use crate::family_pattern::typeclasses::foldable::Foldable;
use crate::family_pattern::typeclasses::functor::{Functor, FunctorMut, FunctorOnce};
use crate::family_pattern::typeclasses::traverse::{Traverse, TraverseMut, TraverseOnce};

pub trait Traversable: Functor + Foldable + Traverse {}
pub trait TraversableMut: FunctorMut + Foldable + TraverseMut {}
pub trait TraversableOnce: FunctorOnce + Foldable + TraverseOnce {}

mod impls {
    use crate::family_pattern::typeclasses::foldable::Foldable;
    use crate::family_pattern::typeclasses::functor::{Functor, FunctorMut, FunctorOnce};
    use crate::family_pattern::typeclasses::traversable::{
        Traversable, TraversableMut, TraversableOnce,
    };
    use crate::family_pattern::typeclasses::traverse::{Traverse, TraverseMut, TraverseOnce};

    impl<T: Functor + Foldable + Traverse> Traversable for T {}
    impl<T: FunctorMut + Foldable + TraverseMut> TraversableMut for T {}
    impl<T: FunctorOnce + Foldable + TraverseOnce> TraversableOnce for T {}
}
