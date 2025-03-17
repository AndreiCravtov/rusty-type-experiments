use crate::family_pattern::{ConstructableTySyntax1, TyConstructor1};
use func::constant;

pub trait Functor: TyConstructor1 {
    fn fmap<A, B, F: Fn(A) -> B>(fa: Self::TC1<A>, f: F) -> Self::TC1<B>;

    #[inline]
    fn fconst<A, B: Clone>(fa: Self::TC1<A>, b: B) -> Self::TC1<B> {
        Self::fmap(fa, constant(b))
    }

    #[inline]
    fn fvoid<A>(fa: Self::TC1<A>) -> Self::TC1<()> {
        Self::fconst(fa, ())
    }
}

pub trait FunctorMut: TyConstructor1 {
    fn fmap_mut<A, B, F: FnMut(A) -> B>(fa: Self::TC1<A>, f: F) -> Self::TC1<B>;
}

pub trait FunctorOnce: TyConstructor1 {
    fn fmap_once<A, B, F: FnOnce(A) -> B>(fa: Self::TC1<A>, f: F) -> Self::TC1<B>;
}

pub trait FunctorSyntax<T: Functor, A>: ConstructableTySyntax1<T, A> {
    fn fmap<B, F: Fn(A) -> B>(self, f: F) -> T::TC1<B>;

    fn fconst<B: Clone>(self, b: B) -> T::TC1<B>;

    fn fvoid(self) -> T::TC1<()>;
}

pub trait FunctorMutSyntax<T: Functor, A>: ConstructableTySyntax1<T, A> {
    fn fmap_mut<B, F: FnMut(A) -> B>(self, f: F) -> T::TC1<B>;
}

pub trait FunctorOnceSyntax<T: Functor, A>: ConstructableTySyntax1<T, A> {
    fn fmap_once<B, F: FnOnce(A) -> B>(self, f: F) -> T::TC1<B>;
}

mod impls {
    use crate::family_pattern::{
        syntax::*, typeclasses::functor::{Functor, FunctorMut, FunctorOnce},
        ConstructableTy1,
        ConstructableTyExt1 as _,
    };

    // blanket `Fn*` reverse-hierarchy implementations
    impl<T: FunctorOnce> FunctorMut for T {
        #[inline]
        fn fmap_mut<A, B, F: FnMut(A) -> B>(fa: Self::TC1<A>, f: F) -> Self::TC1<B> {
            T::fmap_once(fa, f)
        }
    }
    impl<T: FunctorMut> Functor for T {
        #[inline]
        fn fmap<A, B, F: Fn(A) -> B>(fa: Self::TC1<A>, f: F) -> Self::TC1<B> {
            T::fmap_mut(fa, f)
        }
    }

    // blanket `*Syntax` implementations
    impl<T: Functor, A, FA: ConstructableTy1<Constructor = T, GenericParameter1 = A>>
        FunctorSyntax<T, A> for FA
    {
        #[inline]
        fn fmap<B, F: Fn(A) -> B>(self, f: F) -> T::TC1<B> {
            T::fmap(self.reify1(), f)
        }

        #[inline]
        fn fconst<B: Clone>(self, b: B) -> T::TC1<B> {
            T::fconst(self.reify1(), b)
        }

        #[inline]
        fn fvoid(self) -> T::TC1<()> {
            T::fvoid(self.reify1())
        }
    }
    impl<T: FunctorMut, A, FA: ConstructableTy1<Constructor = T, GenericParameter1 = A>>
        FunctorMutSyntax<T, A> for FA
    {
        #[inline]
        fn fmap_mut<B, F: FnMut(A) -> B>(self, f: F) -> T::TC1<B> {
            T::fmap_mut(self.reify1(), f)
        }
    }
    impl<T: FunctorOnce, A, FA: ConstructableTy1<Constructor = T, GenericParameter1 = A>>
        FunctorOnceSyntax<T, A> for FA
    {
        #[inline]
        fn fmap_once<B, F: FnOnce(A) -> B>(self, f: F) -> T::TC1<B> {
            T::fmap_once(self.reify1(), f)
        }
    }
}
