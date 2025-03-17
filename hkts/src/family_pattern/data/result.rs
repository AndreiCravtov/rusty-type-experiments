use std::marker::PhantomData;

/// The type-constructor for the `Result<_,_>` data-type.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub struct ResultConstructor;

// TODO: i don't know its appropriate to have the type-parameter be stored
//       as phantom-data...., perhaps since it is partially applied, it is more
//       appropriate for it to ALREADY hold
/// The type-constructor for the `Result<T,_>` data-type.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
#[repr(transparent)]
pub struct ResultTConstructor<E>(PhantomData<E>);

mod ty2_impls {
    use crate::family_pattern::data::result::ResultConstructor;
    use crate::family_pattern::{ConstructableTy2, TyConstructor2};

    impl<T, E> ConstructableTy2 for Result<T, E> {
        type GenericParameter1 = T;
        type GenericParameter2 = E;
        type Constructor = ResultConstructor;
    }

    impl TyConstructor2 for ResultConstructor {
        type TC2<T, E> = Result<T, E>;
    }
}

mod ty1_impls {
    use crate::family_pattern::data::result::ResultTConstructor;
    use crate::family_pattern::{ConstructableTy1, TyConstructor1};

    impl<T, E> ConstructableTy1 for Result<T, E> {
        type GenericParameter1 = E;
        type Constructor = ResultTConstructor<T>;
    }

    impl<T> TyConstructor1 for ResultTConstructor<T> {
        type TC1<E> = Result<T, E>;
    }

    // TODO: we _could_ implement monad and this and that, but thats booooring....
    //       since mapping over errors is kinds mehhhh,
    //       what we REALLY want is a type-level "flip" operator to s.t `flip(Result<T, E>) = Result<E, T>`
    //       that way partially applying `flip(Result<_,_>)` would create type-constructor `Result<_,E>`
    //       which is what we are REALLY after here....
}
