// // enable Rust-unstable features for convenience
// #![feature(trait_alias)]
// #![feature(stmt_expr_attributes)]
// #![feature(type_alias_impl_trait)]
// #![feature(specialization)]
// #![feature(unboxed_closures)]
// #![feature(const_trait_impl)]
// #![feature(fn_traits)]
// #![feature(non_lifetime_binders)]

pub mod data;

pub(crate) mod private {
    /// Sealed traits support
    pub trait Sealed {}
    impl<T: ?Sized> Sealed for T {}
}

/// Namespace for all the type/trait aliases used by this crate.
pub(crate) mod alias {}

/// Namespace for crate-wide extension traits/methods
pub(crate) mod ext {}
