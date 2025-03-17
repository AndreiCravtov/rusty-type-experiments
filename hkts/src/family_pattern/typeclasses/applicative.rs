use crate::family_pattern::typeclasses::ap::{Ap, ApMut, ApOnce};
use crate::family_pattern::typeclasses::functor::{Functor, FunctorMut, FunctorOnce};
use crate::family_pattern::typeclasses::pure::Pure;

pub trait Applicative: Functor + Pure + Ap {}
pub trait ApplicativeMut: FunctorMut + Pure + ApMut {}
pub trait ApplicativeOnce: FunctorOnce + Pure + ApOnce {}

mod impls {
    use crate::family_pattern::typeclasses::ap::{Ap, ApMut, ApOnce};
    use crate::family_pattern::typeclasses::applicative::{
        Applicative, ApplicativeMut, ApplicativeOnce,
    };
    use crate::family_pattern::typeclasses::functor::{Functor, FunctorMut, FunctorOnce};
    use crate::family_pattern::typeclasses::pure::Pure;

    impl<T: Functor + Pure + Ap> Applicative for T {}
    impl<T: FunctorMut + Pure + ApMut> ApplicativeMut for T {}
    impl<T: FunctorOnce + Pure + ApOnce> ApplicativeOnce for T {}
}
