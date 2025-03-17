use crate::family_pattern::ConstructableTySyntax1;
use crate::family_pattern::TyConstructor1;

pub trait Ap: TyConstructor1 {
    #[inline]
    fn ap<A, B, F: Fn(A) -> B>(ff: Self::TC1<F>, fa: Self::TC1<A>) -> Self::TC1<B> {
        Self::lift_2a(ff, fa, move |f, a| f(a))
    }
    fn lift_2a<A, B, C, F: Fn(A, B) -> C>(fa: Self::TC1<A>, fb: Self::TC1<B>, f: F)
    -> Self::TC1<C>;
}

pub trait ApMut: TyConstructor1 {
    #[inline]
    fn ap_mut<A, B, F: FnMut(A) -> B>(ff: Self::TC1<F>, fa: Self::TC1<A>) -> Self::TC1<B> {
        Self::lift_2a_mut(ff, fa, move |mut f, a| f(a))
    }
    fn lift_2a_mut<A, B, C, F: FnMut(A, B) -> C>(
        fa: Self::TC1<A>,
        fb: Self::TC1<B>,
        f: F,
    ) -> Self::TC1<C>;
}

pub trait ApOnce: TyConstructor1 {
    #[inline]
    fn ap_once<A, B, F: FnOnce(A) -> B>(ff: Self::TC1<F>, fa: Self::TC1<A>) -> Self::TC1<B> {
        Self::lift_2a_once(ff, fa, move |f, a| f(a))
    }
    fn lift_2a_once<A, B, C, F: FnOnce(A, B) -> C>(
        fa: Self::TC1<A>,
        fb: Self::TC1<B>,
        f: F,
    ) -> Self::TC1<C>;
}

pub trait ApSyntax<TC: Ap, A, B, F: Fn(A) -> B>: ConstructableTySyntax1<TC, F> {
    fn ap(self, fa: TC::TC1<A>) -> TC::TC1<B>;
}

pub trait Lift2ASyntax<
    A,
    B,
    T: Ap,
    TA: ConstructableTySyntax1<T, A>,
    TB: ConstructableTySyntax1<T, B>,
>
{
    fn lift_2a<C, F: Fn(A, B) -> C>(self, f: F) -> T::TC1<C>;
}

pub trait ApMutSyntax<TC: ApMut, A, B, F: FnMut(A) -> B>: ConstructableTySyntax1<TC, F> {
    fn ap_mut(self, fa: TC::TC1<A>) -> TC::TC1<B>;
}

pub trait Lift2AMutSyntax<
    A,
    B,
    T: ApMut,
    TA: ConstructableTySyntax1<T, A>,
    TB: ConstructableTySyntax1<T, B>,
>
{
    fn lift_2a_mut<C, F: FnMut(A, B) -> C>(self, f: F) -> T::TC1<C>;
}

pub trait ApOnceSyntax<TC: ApOnce, A, B, F: FnOnce(A) -> B>: ConstructableTySyntax1<TC, F> {
    fn ap_once(self, fa: TC::TC1<A>) -> TC::TC1<B>;
}

pub trait Lift2AOnceSyntax<
    A,
    B,
    T: ApOnce,
    TA: ConstructableTySyntax1<T, A>,
    TB: ConstructableTySyntax1<T, B>,
>
{
    fn lift_2a_once<C, F: FnOnce(A, B) -> C>(self, f: F) -> T::TC1<C>;
}
mod impls {
    use crate::family_pattern::{
        syntax::*, typeclasses::ap::{Ap, ApMut, ApOnce},
        ConstructableTy1,
        ConstructableTyExt1 as _,
    };

    // blanket `Fn*` reverse-hierarchy implementations
    impl<TC: ApOnce> ApMut for TC {
        #[inline]
        fn ap_mut<A, B, F: FnMut(A) -> B>(ff: Self::TC1<F>, fa: Self::TC1<A>) -> Self::TC1<B> {
            TC::ap_once(ff, fa)
        }

        #[inline]
        fn lift_2a_mut<A, B, C, F: FnMut(A, B) -> C>(
            fa: Self::TC1<A>,
            fb: Self::TC1<B>,
            f: F,
        ) -> Self::TC1<C> {
            TC::lift_2a_once(fa, fb, f)
        }
    }
    impl<TC: ApMut> Ap for TC {
        #[inline]
        fn ap<A, B, F: Fn(A) -> B>(ff: Self::TC1<F>, fa: Self::TC1<A>) -> Self::TC1<B> {
            TC::ap_mut(ff, fa)
        }

        #[inline]
        fn lift_2a<A, B, C, F: FnMut(A, B) -> C>(
            fa: Self::TC1<A>,
            fb: Self::TC1<B>,
            f: F,
        ) -> Self::TC1<C> {
            TC::lift_2a_mut(fa, fb, f)
        }
    }

    // blanket `*Syntax` implementations
    impl<TC: Ap, A, B, F: Fn(A) -> B, FA: ConstructableTy1<Constructor = TC, GenericParameter1 = F>>
        ApSyntax<TC, A, B, F> for FA
    {
        #[inline]
        fn ap(self, fa: TC::TC1<A>) -> TC::TC1<B> {
            TC::ap(self.reify1(), fa)
        }
    }
    impl<A, B, T: Ap, TA: ConstructableTySyntax1<T, A>, TB: ConstructableTySyntax1<T, B>>
        Lift2ASyntax<A, B, T, TA, TB> for (TA, TB)
    {
        #[inline]
        fn lift_2a<C, F: Fn(A, B) -> C>(self, f: F) -> T::TC1<C> {
            T::lift_2a(self.0.reify1(), self.1.reify1(), f)
        }
    }
    impl<
        TC: ApMut,
        A,
        B,
        F: FnMut(A) -> B,
        FA: ConstructableTy1<Constructor = TC, GenericParameter1 = F>,
    > ApMutSyntax<TC, A, B, F> for FA
    {
        #[inline]
        fn ap_mut(self, fa: TC::TC1<A>) -> TC::TC1<B> {
            TC::ap_mut(self.reify1(), fa)
        }
    }
    impl<A, B, T: ApMut, TA: ConstructableTySyntax1<T, A>, TB: ConstructableTySyntax1<T, B>>
        Lift2AMutSyntax<A, B, T, TA, TB> for (TA, TB)
    {
        #[inline]
        fn lift_2a_mut<C, F: FnMut(A, B) -> C>(self, f: F) -> T::TC1<C> {
            T::lift_2a_mut(self.0.reify1(), self.1.reify1(), f)
        }
    }
    impl<
        TC: ApOnce,
        A,
        B,
        F: FnOnce(A) -> B,
        FA: ConstructableTy1<Constructor = TC, GenericParameter1 = F>,
    > ApOnceSyntax<TC, A, B, F> for FA
    {
        #[inline]
        fn ap_once(self, fa: TC::TC1<A>) -> TC::TC1<B> {
            TC::ap_once(self.reify1(), fa)
        }
    }
    impl<A, B, T: ApOnce, TA: ConstructableTySyntax1<T, A>, TB: ConstructableTySyntax1<T, B>>
        Lift2AOnceSyntax<A, B, T, TA, TB> for (TA, TB)
    {
        #[inline]
        fn lift_2a_once<C, F: FnOnce(A, B) -> C>(self, f: F) -> T::TC1<C> {
            T::lift_2a_once(self.0.reify1(), self.1.reify1(), f)
        }
    }
}
