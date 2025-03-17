use crate::family_pattern::{
    syntax::*, typeclasses::{
        ap::ApOnce, applicative::Applicative, bind::BindOnce, foldable::FoldableOnce,
        functor::FunctorOnce, monoid::Monoid, pure::Pure, semigroup::Semigroup,
        traverse::TraverseOnce,
    },
    ConstructableTy1,
    TyConstructor1,
};

/// The type-constructor for the `Option<_>` data-type.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub struct OptionConstructor;

impl<T> ConstructableTy1 for Option<T> {
    type GenericParameter1 = T;
    type Constructor = OptionConstructor;
}

impl TyConstructor1 for OptionConstructor {
    type TC1<T> = Option<T>;
}

impl<T: Semigroup> Semigroup for Option<T> {
    #[inline]
    fn scombine(self, other: Self) -> Self {
        match (self, other) {
            (None, b) => b,
            (a, None) => a,
            (Some(a), Some(b)) => Some(a.scombine(b)),
        }
    }
}

impl<T: Semigroup> Monoid for Option<T> {
    #[inline]
    fn mempty() -> Self {
        None
    }
}

impl FunctorOnce for OptionConstructor {
    #[inline]
    fn fmap_once<A, B, F: FnOnce(A) -> B>(fa: Option<A>, f: F) -> Option<B> {
        fa.map(f)
    }
}

impl Pure for OptionConstructor {
    #[inline]
    fn pure<A>(a: A) -> Option<A> {
        Some(a)
    }
}

impl ApOnce for OptionConstructor {
    #[inline]
    fn lift_2a_once<A, B, C, F: FnOnce(A, B) -> C>(
        fa: Self::TC1<A>,
        fb: Self::TC1<B>,
        f: F,
    ) -> Self::TC1<C> {
        match (fa, fb) {
            (Some(a), Some(b)) => Some(f(a, b)),
            _ => None,
        }
    }
}

impl BindOnce for OptionConstructor {
    #[inline]
    fn bind_once<A, B, F: FnOnce(A) -> Option<B>>(ta: Option<A>, f: F) -> Option<B> {
        ta.and_then(f)
    }
}

// impl MonadTry for OptionConstructor {
//     #[inline]
//     fn into_control_flow<A>(ma: Option<A>) -> ControlFlow<Option<convert::Infallible>, A> {
//         ma.branch()
//     }
// }

impl FoldableOnce for OptionConstructor {
    #[inline]
    fn foldr_once<A, S, F: FnOnce(A, S) -> S>(ta: Option<A>, s: S, f: F) -> S {
        match ta {
            None => s,
            Some(a) => f(a, s),
        }
    }

    #[inline]
    fn foldl_once<A, S, F: FnOnce(S, A) -> S>(ta: Option<A>, s: S, f: F) -> S {
        match ta {
            None => s,
            Some(a) => f(s, a),
        }
    }
}

impl TraverseOnce for OptionConstructor {
    #[inline]
    fn traverse_once<F: Applicative, A, B, _F: FnOnce(A) -> F::TC1<B>>(
        ta: Option<A>,
        f: _F,
    ) -> F::TC1<Option<B>> {
        ta.map_or_else(|| F::pure(None), |a| f(a).fmap(Some))
    }
}
