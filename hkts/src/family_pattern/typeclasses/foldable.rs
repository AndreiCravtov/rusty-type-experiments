use crate::family_pattern::typeclasses::monoid::Monoid;
use crate::family_pattern::ConstructableTySyntax1;
use crate::family_pattern::TyConstructor1;

pub trait Foldable: TyConstructor1 {
    fn foldr<A, S, F: Fn(A, S) -> S>(ta: Self::TC1<A>, s: S, f: F) -> S;

    fn foldl<A, S, F: Fn(S, A) -> S>(ta: Self::TC1<A>, s: S, f: F) -> S;

    #[inline]
    fn foldr_map<A, M: Monoid, F: Fn(A) -> M>(ta: Self::TC1<A>, f: F) -> M {
        Self::foldr(ta, M::mempty(), |a, s| f(a).scombine(s))
    }

    #[inline]
    fn foldl_map<A, M: Monoid, F: Fn(A) -> M>(ta: Self::TC1<A>, f: F) -> M {
        Self::foldl(ta, M::mempty(), |s, a| s.scombine(f(a)))
    }
}

pub trait FoldableMut: TyConstructor1 {
    fn foldr_mut<A, S, F: FnMut(A, S) -> S>(ta: Self::TC1<A>, s: S, f: F) -> S;

    fn foldl_mut<A, S, F: FnMut(S, A) -> S>(ta: Self::TC1<A>, s: S, f: F) -> S;

    #[inline]
    fn foldr_map_mut<A, M: Monoid, F: FnMut(A) -> M>(ta: Self::TC1<A>, mut f: F) -> M {
        Self::foldr_mut(ta, M::mempty(), |a, s| f(a).scombine(s))
    }

    #[inline]
    fn foldl_map_mut<A, M: Monoid, F: FnMut(A) -> M>(ta: Self::TC1<A>, mut f: F) -> M {
        Self::foldl_mut(ta, M::mempty(), |s, a| s.scombine(f(a)))
    }
}

pub trait FoldableOnce: TyConstructor1 {
    fn foldr_once<A, S, F: FnOnce(A, S) -> S>(ta: Self::TC1<A>, s: S, f: F) -> S;

    fn foldl_once<A, S, F: FnOnce(S, A) -> S>(ta: Self::TC1<A>, s: S, f: F) -> S;

    #[inline]
    fn foldr_map_once<A, M: Monoid, F: FnOnce(A) -> M>(ta: Self::TC1<A>, f: F) -> M {
        Self::foldr_once(ta, M::mempty(), |a, s| f(a).scombine(s))
    }

    #[inline]
    fn foldl_map_once<A, M: Monoid, F: FnOnce(A) -> M>(ta: Self::TC1<A>, f: F) -> M {
        Self::foldl_once(ta, M::mempty(), |s, a| s.scombine(f(a)))
    }
}

pub trait FoldableSyntax<T: Foldable, A>: ConstructableTySyntax1<T, A> {
    fn foldr<S, F: Fn(A, S) -> S>(self, s: S, f: F) -> S;

    fn foldl<S, F: Fn(S, A) -> S>(self, s: S, f: F) -> S;

    fn foldr_map<M: Monoid, F: Fn(A) -> M>(self, f: F) -> M;

    fn foldl_map<M: Monoid, F: Fn(A) -> M>(self, f: F) -> M;
}

pub trait FoldableMutSyntax<T: Foldable, A>: ConstructableTySyntax1<T, A> {
    fn foldr_mut<S, F: FnMut(A, S) -> S>(self, s: S, f: F) -> S;

    fn foldl_mut<S, F: FnMut(S, A) -> S>(self, s: S, f: F) -> S;

    fn foldr_map_mut<M: Monoid, F: FnMut(A) -> M>(self, f: F) -> M;

    fn foldl_map_mut<M: Monoid, F: FnMut(A) -> M>(self, f: F) -> M;
}

pub trait FoldableOnceSyntax<T: Foldable, A>: ConstructableTySyntax1<T, A> {
    fn foldr_once<S, F: FnOnce(A, S) -> S>(self, s: S, f: F) -> S;

    fn foldl_once<S, F: FnOnce(S, A) -> S>(self, s: S, f: F) -> S;

    fn foldr_map_once<M: Monoid, F: FnOnce(A) -> M>(self, f: F) -> M;

    fn foldl_map_once<M: Monoid, F: FnOnce(A) -> M>(self, f: F) -> M;
}

mod impls {
    use crate::family_pattern::{
        syntax::*, typeclasses::{
            foldable::{Foldable, FoldableMut, FoldableOnce},
            monoid::Monoid,
        },
        ConstructableTy1,
        ConstructableTyExt1 as _,
    };

    // blanket `Fn*` reverse-hierarchy implementations
    impl<T: FoldableOnce> FoldableMut for T {
        #[inline]
        fn foldr_mut<A, S, F: FnMut(A, S) -> S>(ta: Self::TC1<A>, s: S, f: F) -> S {
            T::foldr_once(ta, s, f)
        }

        #[inline]
        fn foldl_mut<A, S, F: FnMut(S, A) -> S>(ta: Self::TC1<A>, s: S, f: F) -> S {
            T::foldl_once(ta, s, f)
        }

        #[inline]
        fn foldr_map_mut<A, M: Monoid, F: FnMut(A) -> M>(ta: Self::TC1<A>, f: F) -> M {
            T::foldr_map_once(ta, f)
        }

        #[inline]
        fn foldl_map_mut<A, M: Monoid, F: FnMut(A) -> M>(ta: Self::TC1<A>, f: F) -> M {
            T::foldl_map_once(ta, f)
        }
    }
    impl<T: FoldableMut> Foldable for T {
        #[inline]
        fn foldr<A, S, F: FnMut(A, S) -> S>(ta: Self::TC1<A>, s: S, f: F) -> S {
            T::foldr_mut(ta, s, f)
        }

        #[inline]
        fn foldl<A, S, F: FnMut(S, A) -> S>(ta: Self::TC1<A>, s: S, f: F) -> S {
            T::foldl_mut(ta, s, f)
        }

        #[inline]
        fn foldr_map<A, M: Monoid, F: FnMut(A) -> M>(ta: Self::TC1<A>, f: F) -> M {
            T::foldr_map_mut(ta, f)
        }

        #[inline]
        fn foldl_map<A, M: Monoid, F: FnMut(A) -> M>(ta: Self::TC1<A>, f: F) -> M {
            T::foldl_map_mut(ta, f)
        }
    }

    // blanket `*Syntax` implementations
    impl<T: Foldable, A, TA: ConstructableTy1<Constructor = T, GenericParameter1 = A>>
        FoldableSyntax<T, A> for TA
    {
        #[inline]
        fn foldr<S, F: Fn(A, S) -> S>(self, s: S, f: F) -> S {
            T::foldr(self.reify1(), s, f)
        }

        #[inline]
        fn foldl<S, F: Fn(S, A) -> S>(self, s: S, f: F) -> S {
            T::foldl(self.reify1(), s, f)
        }

        #[inline]
        fn foldr_map<B: Monoid, F: Fn(A) -> B>(self, f: F) -> B {
            T::foldr_map(self.reify1(), f)
        }

        #[inline]
        fn foldl_map<B: Monoid, F: Fn(A) -> B>(self, f: F) -> B {
            T::foldl_map(self.reify1(), f)
        }
    }
    impl<T: FoldableMut, A, TA: ConstructableTy1<Constructor = T, GenericParameter1 = A>>
        FoldableMutSyntax<T, A> for TA
    {
        #[inline]
        fn foldr_mut<S, F: FnMut(A, S) -> S>(self, s: S, f: F) -> S {
            T::foldr_mut(self.reify1(), s, f)
        }

        #[inline]
        fn foldl_mut<S, F: FnMut(S, A) -> S>(self, s: S, f: F) -> S {
            T::foldl_mut(self.reify1(), s, f)
        }

        #[inline]
        fn foldr_map_mut<B: Monoid, F: FnMut(A) -> B>(self, f: F) -> B {
            T::foldr_map_mut(self.reify1(), f)
        }

        #[inline]
        fn foldl_map_mut<B: Monoid, F: FnMut(A) -> B>(self, f: F) -> B {
            T::foldl_map_mut(self.reify1(), f)
        }
    }
    impl<T: FoldableOnce, A, TA: ConstructableTy1<Constructor = T, GenericParameter1 = A>>
        FoldableOnceSyntax<T, A> for TA
    {
        #[inline]
        fn foldr_once<S, F: FnOnce(A, S) -> S>(self, s: S, f: F) -> S {
            T::foldr_once(self.reify1(), s, f)
        }

        #[inline]
        fn foldl_once<S, F: FnOnce(S, A) -> S>(self, s: S, f: F) -> S {
            T::foldl_once(self.reify1(), s, f)
        }

        #[inline]
        fn foldr_map_once<B: Monoid, F: FnOnce(A) -> B>(self, f: F) -> B {
            T::foldr_map_once(self.reify1(), f)
        }

        #[inline]
        fn foldl_map_once<B: Monoid, F: FnOnce(A) -> B>(self, f: F) -> B {
            T::foldl_map_once(self.reify1(), f)
        }
    }
}
