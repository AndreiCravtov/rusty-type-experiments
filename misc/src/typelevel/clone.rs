use crate::typelevel::typewitness::With;
use std::marker::PhantomData;

/// A type [`T`] together with type-witness [`CloneWit`] that it implements [`Clone`].
pub type WithClone<T> = With<T, CloneWit<T>>;

/// This is a type-witness for [`Clone`], it can only be constructed for types that implement [`Clone`].
#[repr(transparent)]
pub struct CloneWit<T: ?Sized>(PhantomData<T>);

#[inline]
#[must_use]
pub const fn is_clone<T: Clone>() -> CloneWit<T> {
    CloneWit::new()
}

/// This is an instance provider trait for [`Clone`], it uses specialization to provide the correct
/// implementation of [`Clone`] with static-dispatch, based on a supplied [`CloneWit`] witness value
pub trait CloneInstance {
    // TODO: change to generic type that takes "CloneWit" provider, or something....
    //       CloneWit: Provider<CloneWit>
    //       HList<??>: Contains<CloneWit>  => HList<??>: Provider<CloneWit>
    //       TypeWitness::WitnessesHList<??>: Contains<CloneWit> => TypeWitness: Provider<CloneWit>
    // TODO: and perhaps `impl<T> Provider<T> for &T` blanket impl??
    #[must_use]
    fn clone_with(&self, _wit: &CloneWit<Self>) -> Self
    where
        Self: Sized;
}

mod impls {
    #![allow(clippy::inline_always)]

    use crate::typelevel::clone::{CloneInstance, CloneWit, WithClone};
    use crate::typelevel::typewitness::TypeWitness;
    use std::cmp::Ordering;
    use std::hash::{Hash, Hasher};
    use std::marker::PhantomData;

    impl<T: ?Sized> CloneWit<T> {
        #[inline(always)]
        #[must_use]
        pub const fn new() -> Self
        where
            T: Clone,
        {
            Self(PhantomData)
        }
    }

    // TODO: create proc-macro for deriving these implementations, something like
    //       ```rust
    //       #[derive(Witness)]
    //       #[wit_fn(is)] struct IsWit<L,R> where L: Is<Ty=R>;
    //       ```
    //       would e.g.:
    //       1) inspect type-variables L,R
    //       2) inspect type-constraints L: Is<Ty=R>
    //       3) replace body with `(modifier) struct IsWit<L: ?Sized, R: ?Sized>(PhantomData<L>, PhantomData<R>);`
    //          and add `#[repr(transparent)]` if possible
    //       4) generate trait-impls that don't need trait bounds: Hash, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, TypeWitness
    //       5) generate impl-block with "new" method that has the appropriate trait-bounds on it collected earlier
    //       6) generate default-trait impl which has the appropriate trait-bounds on it collected earlier
    //       7) if `wit_fn` parameter found, generate function `const fn is<..>() -> .. {}` which would be a shorthand
    //          witness "constructor" with the right trait-bounds and have the same visibility modifier as `IsWit`
    impl<T: ?Sized> Hash for CloneWit<T> {
        #[inline]
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }

    impl<T: ?Sized> PartialEq for CloneWit<T> {
        #[inline]
        fn eq(&self, other: &Self) -> bool {
            self.0.eq(&other.0)
        }
    }

    impl<T: ?Sized> Eq for CloneWit<T> {}

    #[allow(clippy::non_canonical_partial_ord_impl)]
    impl<T: ?Sized> PartialOrd for CloneWit<T> {
        #[inline]
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.0.partial_cmp(&other.0)
        }
    }

    impl<T: ?Sized> Ord for CloneWit<T> {
        #[inline]
        fn cmp(&self, other: &Self) -> Ordering {
            self.0.cmp(&other.0)
        }
    }

    impl<T: ?Sized> Copy for CloneWit<T> {}

    #[allow(clippy::non_canonical_clone_impl)]
    impl<T: ?Sized> Clone for CloneWit<T> {
        #[inline]
        fn clone(&self) -> Self {
            Self(self.0)
        }
    }

    impl<T: Clone> Default for CloneWit<T> {
        #[inline(always)]
        fn default() -> Self {
            Self::new()
        }
    }

    impl<T: ?Sized> TypeWitness for CloneWit<T> {}

    impl<T: ?Sized> CloneInstance for T {
        #[inline(always)]
        default fn clone_with(&self, _wit: &CloneWit<Self>) -> Self
        where
            Self: Sized,
        {
            unreachable!()
        }
    }
    impl<T: Clone> CloneInstance for T {
        #[inline(always)]
        fn clone_with(&self, _wit: &CloneWit<Self>) -> Self
        where
            Self: Sized,
        {
            self.clone()
        }
    }

    // TODO: right now, implementing traits on this is rather awkward, as the result is "wrapped"
    //       e.g. a naiive implementation of `impl<T> Clone for With<T, CloneWit>` would be
    //            `&With<T, CloneWit> -> With<T, CloneWit>` where we __really__ want something like
    //            `&With<T, CloneWit> -> T` or more generally I want `With<T, CloneWit>` to be a RECEIVER
    //            for type `T` - but only sometimes?? idk...
    impl<T> Clone for WithClone<T> {
        #[inline]
        fn clone(&self) -> Self {
            let (value, witness) = self.tuple();
            Self::new(value.clone_with(&witness), witness)
        }
    }
}
