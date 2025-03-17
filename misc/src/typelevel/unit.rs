use crate::typelevel::typewitness::With;
use std::marker::PhantomData;

/// The [unit-type](https://en.wikipedia.org/wiki/Unit_type)-equivalent for traits, implemented for all types [`T`].
/// It is meant to represent _no trait bounds whatsoever_ in case _some_ trait-bound type-witness is needed.
#[allow(clippy::too_long_first_doc_paragraph)]
pub trait Unit {}

/// A type [`T`] together with type-witness [`UnitWit`] that it implements [`Unit`].
pub type WithUnit<T> = With<T, UnitWit<T>>;

/// This is a type-witness for [`Unit`], it can be constructed for all types (since they all implement [`Unit`]).
#[repr(transparent)]
pub struct UnitWit<T: ?Sized>(PhantomData<T>);

#[inline]
#[must_use]
pub const fn is_unit<T: ?Sized>() -> UnitWit<T> {
    UnitWit::new()
}

mod impls {
    #![allow(clippy::inline_always)]

    use crate::typelevel::typewitness::TypeWitness;
    use crate::typelevel::unit::{Unit, UnitWit};
    use std::cmp::Ordering;
    use std::hash::{Hash, Hasher};
    use std::marker::PhantomData;

    impl<T: ?Sized> Unit for T {}

    impl<T: ?Sized> UnitWit<T> {
        #[inline(always)]
        #[must_use]
        pub const fn new() -> Self {
            Self(PhantomData)
        }
    }

    impl<T: ?Sized> Hash for UnitWit<T> {
        #[inline]
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }

    impl<T: ?Sized> PartialEq for UnitWit<T> {
        #[inline]
        fn eq(&self, other: &Self) -> bool {
            self.0.eq(&other.0)
        }
    }

    impl<T: ?Sized> Eq for UnitWit<T> {}

    #[allow(clippy::non_canonical_partial_ord_impl)]
    impl<T: ?Sized> PartialOrd for UnitWit<T> {
        #[inline]
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.0.partial_cmp(&other.0)
        }
    }

    impl<T: ?Sized> Ord for UnitWit<T> {
        #[inline]
        fn cmp(&self, other: &Self) -> Ordering {
            self.0.cmp(&other.0)
        }
    }

    impl<T: ?Sized> Copy for UnitWit<T> {}

    #[allow(clippy::non_canonical_clone_impl)]
    impl<T: ?Sized> Clone for UnitWit<T> {
        #[inline]
        fn clone(&self) -> Self {
            Self(self.0)
        }
    }

    impl<T: ?Sized> Default for UnitWit<T> {
        #[inline(always)]
        fn default() -> Self {
            Self::new()
        }
    }

    impl<T: ?Sized> TypeWitness for UnitWit<T> {}
}
