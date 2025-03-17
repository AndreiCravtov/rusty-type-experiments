use crate::family_pattern::ConstructableTySyntax1;
use crate::family_pattern::TyConstructor1;

pub trait Pure: TyConstructor1 {
    fn pure<A>(a: A) -> Self::TC1<A>;
}

pub trait PureSyntax<T: Pure, A> {
    fn pure<TA: ConstructableTySyntax1<T, A>>(self) -> T::TC1<A>;
}

mod impls {
    use crate::family_pattern::{syntax::*, typeclasses::pure::Pure};

    impl<T: Pure, A> PureSyntax<T, A> for A {
        #[inline]
        fn pure<TA: ConstructableTySyntax1<T, A>>(self) -> T::TC1<A> {
            T::pure(self)
        }
    }
}
