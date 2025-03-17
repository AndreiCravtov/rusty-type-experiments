//! Family-pattern encoding of HKTs inspired by this [ratz](https://github.com/mschuwalow/ratz/) repository.
//!

pub mod data;

pub mod flip;
pub mod into;
pub mod ty_constructor;
pub mod typeclasses;

pub use re_exports::*;

mod re_exports {
    pub use crate::family_pattern::ty_constructor::alias::*;
    pub use crate::family_pattern::ty_constructor::*;
}

pub mod syntax {
    pub use crate::family_pattern::{
        ty_constructor::alias::{
            ConstructableTySyntax1, ConstructableTySyntax2, ConstructableTySyntax3,
        },
        typeclasses::{
            ap::{
                ApMutSyntax, ApOnceSyntax, ApSyntax, Lift2AMutSyntax, Lift2AOnceSyntax,
                Lift2ASyntax,
            },
            bind::{BindMutSyntax, BindOnceSyntax, BindSyntax},
            foldable::{FoldableMutSyntax, FoldableOnceSyntax, FoldableSyntax},
            functor::{FunctorMutSyntax, FunctorOnceSyntax, FunctorSyntax},
            pure::PureSyntax,
            traverse::{TraverseMutSyntax, TraverseOnceSyntax, TraverseSyntax},
        },
    };
}
