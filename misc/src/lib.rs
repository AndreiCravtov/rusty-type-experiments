// enable Rust-unstable features for convenience
#![feature(trait_alias)]
#![feature(stmt_expr_attributes)]
#![feature(type_alias_impl_trait)]
#![feature(specialization)]
#![feature(unboxed_closures)]
#![feature(const_trait_impl)]
#![feature(fn_traits)]
#![feature(non_lifetime_binders)]

mod dyn_traits;
pub mod func;
mod meow_restricting;
pub mod recursion;
mod rust_gadt_playground;
pub mod typelevel;

pub(crate) mod private {
    /// Sealed traits support
    pub trait Sealed {}
    impl<T: ?Sized> Sealed for T {}
}

/// Namespace for all the type/trait aliases used by this crate.
pub mod alias {
    use std::fmt::Debug;
    use std::hash::Hash;

    pub trait FullDerive = Copy + Clone + PartialEq + Eq + Hash + Debug + PartialOrd + Ord;
}

/// Namespace for crate-wide extension traits/methods
pub mod ext {
    use extend::ext;
    use std::mem;

    #[ext(pub, name = BoxedSliceExt)]
    impl<T> Box<[T]> {
        #[inline]
        fn map<B, F>(self, f: F) -> Box<[B]>
        where
            F: FnMut(T) -> B,
        {
            self.into_iter().map(f).collect()
        }
    }

    #[ext(pub, name = VecExt)]
    impl<T> Vec<T> {
        #[inline]
        fn map<B, F>(self, f: F) -> Vec<B>
        where
            F: FnMut(T) -> B,
        {
            self.into_iter().map(f).collect()
        }
    }

    #[ext(pub, name = UniversalExt)]
    impl<T> T {
        /// Dangerously casts this value of type [`T`] to target type [`U`], without performing
        /// _**any**_ checks.
        ///
        /// This is _**really**_ dangerous!!! Make sure that _**transmuting**_ this value to the
        /// target type is safe.
        #[inline]
        unsafe fn dangerous_unchecked_cast<U>(self) -> U {
            // SAFETY: As per this function's documentation, whoever calls this function
            //         is first ensuring that performing the transmute is safe.
            unsafe { mem::transmute_copy(&mem::ManuallyDrop::new(self)) }
        }
    }
}
