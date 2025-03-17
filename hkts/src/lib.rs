// enable Rust-unstable features for convenience
#![feature(trait_alias)]
#![feature(associated_type_defaults)]
#![feature(const_trait_impl)]
#![feature(try_trait_v2)]

pub mod family_pattern;

pub(crate) mod private {

    /// Sealed traits support for this crate
    pub trait Sealed {}
    impl<T: ?Sized> Sealed for T {}
}

/// Namespace for crate-wide extension traits/methods
pub(crate) mod ext {}
