use crate::family_pattern::typeclasses::applicative::{
    Applicative, ApplicativeMut, ApplicativeOnce,
};
use crate::family_pattern::ConstructableTySyntax1;
use crate::family_pattern::TyConstructor1;

pub trait Traverse: TyConstructor1 {
    fn traverse<F: ApplicativeOnce, A, B, _F: Fn(A) -> F::TC1<B>>(
        ta: Self::TC1<A>,
        f: _F,
    ) -> F::TC1<Self::TC1<B>>;
}

pub trait TraverseMut: TyConstructor1 {
    fn traverse_mut<F: ApplicativeMut, A, B, _F: FnMut(A) -> F::TC1<B>>(
        ta: Self::TC1<A>,
        f: _F,
    ) -> F::TC1<Self::TC1<B>>;
}

pub trait TraverseOnce: TyConstructor1 {
    fn traverse_once<F: Applicative, A, B, _F: FnOnce(A) -> F::TC1<B>>(
        ta: Self::TC1<A>,
        f: _F,
    ) -> F::TC1<Self::TC1<B>>;
}

pub trait TraverseSyntax<TC: Traverse, A>: ConstructableTySyntax1<TC, A> {
    fn traverse<F: ApplicativeOnce, B, _F: Fn(A) -> F::TC1<B>>(self, f: _F) -> F::TC1<TC::TC1<B>>;
}

pub trait TraverseMutSyntax<TC: Traverse, A>: ConstructableTySyntax1<TC, A> {
    fn traverse_mut<F: ApplicativeMut, B, _F: FnMut(A) -> F::TC1<B>>(
        self,
        f: _F,
    ) -> F::TC1<TC::TC1<B>>;
}

pub trait TraverseOnceSyntax<TC: Traverse, A>: ConstructableTySyntax1<TC, A> {
    fn traverse_once<F: Applicative, B, _F: FnOnce(A) -> F::TC1<B>>(
        self,
        f: _F,
    ) -> F::TC1<TC::TC1<B>>;
}

mod impls {
    use crate::family_pattern::typeclasses::applicative::{ApplicativeMut, ApplicativeOnce};
    use crate::family_pattern::{
        syntax::*, typeclasses::applicative::Applicative,
        typeclasses::traverse::{Traverse, TraverseMut, TraverseOnce},
        ConstructableTy1,
        ConstructableTyExt1 as _,
    };

    // blanket `Fn*` reverse-hierarchy implementations
    impl<TC: TraverseOnce> TraverseMut for TC {
        #[inline]
        fn traverse_mut<F: ApplicativeMut, A, B, _F: FnMut(A) -> F::TC1<B>>(
            ta: Self::TC1<A>,
            f: _F,
        ) -> F::TC1<Self::TC1<B>> {
            TC::traverse_once::<F, A, B, _F>(ta, f)
        }
    }
    impl<TC: TraverseMut> Traverse for TC {
        #[inline]
        fn traverse<F: ApplicativeOnce, A, B, _F: Fn(A) -> F::TC1<B>>(
            ta: Self::TC1<A>,
            f: _F,
        ) -> F::TC1<Self::TC1<B>> {
            TC::traverse_mut::<F, A, B, _F>(ta, f)
        }
    }

    // blanket `*Syntax` implementations
    impl<TC: Traverse, A, FA: ConstructableTy1<Constructor = TC, GenericParameter1 = A>>
        TraverseSyntax<TC, A> for FA
    {
        #[inline]
        fn traverse<F: ApplicativeOnce, B, _F: Fn(A) -> F::TC1<B>>(
            self,
            f: _F,
        ) -> F::TC1<TC::TC1<B>> {
            TC::traverse::<F, A, B, _F>(self.reify1(), f)
        }
    }
    impl<TC: TraverseMut, A, FA: ConstructableTy1<Constructor = TC, GenericParameter1 = A>>
        TraverseMutSyntax<TC, A> for FA
    {
        #[inline]
        fn traverse_mut<F: ApplicativeMut, B, _F: FnMut(A) -> F::TC1<B>>(
            self,
            f: _F,
        ) -> F::TC1<TC::TC1<B>> {
            TC::traverse_mut::<F, A, B, _F>(self.reify1(), f)
        }
    }
    impl<TC: TraverseOnce, A, FA: ConstructableTy1<Constructor = TC, GenericParameter1 = A>>
        TraverseOnceSyntax<TC, A> for FA
    {
        #[inline]
        fn traverse_once<F: Applicative, B, _F: FnOnce(A) -> F::TC1<B>>(
            self,
            f: _F,
        ) -> F::TC1<TC::TC1<B>> {
            TC::traverse_once::<F, A, B, _F>(self.reify1(), f)
        }
    }
}
