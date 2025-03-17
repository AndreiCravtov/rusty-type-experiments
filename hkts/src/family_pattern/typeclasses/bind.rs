use crate::family_pattern::ConstructableTySyntax1;
use crate::family_pattern::TyConstructor1;

pub trait Bind: TyConstructor1 {
    fn bind<A, B, F: Fn(A) -> Self::TC1<B>>(ta: Self::TC1<A>, f: F) -> Self::TC1<B>;
}

pub trait BindMut: TyConstructor1 {
    fn bind_mut<A, B, F: FnMut(A) -> Self::TC1<B>>(ta: Self::TC1<A>, f: F) -> Self::TC1<B>;
}

pub trait BindOnce: TyConstructor1 {
    fn bind_once<A, B, F: FnOnce(A) -> Self::TC1<B>>(ta: Self::TC1<A>, f: F) -> Self::TC1<B>;
}

pub trait BindSyntax<T: Bind, A>: ConstructableTySyntax1<T, A> {
    fn bind<B, F: Fn(A) -> T::TC1<B>>(self, f: F) -> T::TC1<B>;
}

pub trait BindMutSyntax<T: BindMut, A>: ConstructableTySyntax1<T, A> {
    fn bind_mut<B, F: FnMut(A) -> T::TC1<B>>(self, f: F) -> T::TC1<B>;
}

pub trait BindOnceSyntax<T: BindOnce, A>: ConstructableTySyntax1<T, A> {
    fn bind_once<B, F: FnOnce(A) -> T::TC1<B>>(self, f: F) -> T::TC1<B>;
}

mod impls {
    use crate::family_pattern::{
        syntax::*, typeclasses::bind::{Bind, BindMut, BindOnce},
        ConstructableTy1,
        ConstructableTyExt1 as _,
    };

    // blanket `Fn*` reverse-hierarchy implementations
    impl<T: BindOnce> BindMut for T {
        #[inline]
        fn bind_mut<A, B, F: FnMut(A) -> Self::TC1<B>>(ta: Self::TC1<A>, f: F) -> Self::TC1<B> {
            T::bind_once(ta, f)
        }
    }
    impl<T: BindMut> Bind for T {
        #[inline]
        fn bind<A, B, F: Fn(A) -> Self::TC1<B>>(ta: Self::TC1<A>, f: F) -> Self::TC1<B> {
            T::bind_mut(ta, f)
        }
    }

    // blanket `*Syntax` implementations
    impl<T: Bind, A, FA: ConstructableTy1<Constructor = T, GenericParameter1 = A>> BindSyntax<T, A>
        for FA
    {
        #[inline]
        fn bind<B, F: Fn(A) -> T::TC1<B>>(self, f: F) -> T::TC1<B> {
            T::bind(self.reify1(), f)
        }
    }
    impl<T: BindMut, A, FA: ConstructableTy1<Constructor = T, GenericParameter1 = A>>
        BindMutSyntax<T, A> for FA
    {
        #[inline]
        fn bind_mut<B, F: FnMut(A) -> T::TC1<B>>(self, f: F) -> T::TC1<B> {
            T::bind_mut(self.reify1(), f)
        }
    }
    impl<T: BindOnce, A, FA: ConstructableTy1<Constructor = T, GenericParameter1 = A>>
        BindOnceSyntax<T, A> for FA
    {
        #[inline]
        fn bind_once<B, F: FnOnce(A) -> T::TC1<B>>(self, f: F) -> T::TC1<B> {
            T::bind_once(self.reify1(), f)
        }
    }
}
