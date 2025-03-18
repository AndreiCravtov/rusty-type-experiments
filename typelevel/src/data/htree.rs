#![allow(
    clippy::same_name_method,
    clippy::inline_always,
    clippy::mismatching_type_param_order,
    clippy::renamed_function_params,
    clippy::too_long_first_doc_paragraph
)]

//! Module that holds `HTree` data structures, implementations, and trait.

/// Trait for HTree-y behaviour
///
/// An `HTree` is a heterogeneous binary tree, one that is statically typed at compile time.
pub trait HTree: Sized {
    /// A `HLeaf` node is a `HFork` with precisely two `HTip` subtrees, i.e. no left or right children.
    const LEAF: bool;

    /// A full binary `HTree` is either:
    /// - A leaf node
    /// - A tree with a root that has two subtrees, both of which are full binary trees.
    const FULL: bool;

    /// A perfect binary `HTree` is a [`Self::FULL`] binary `HTree` that has the same number of
    /// edges from the root node to any of its leaf nodes.
    const PERFECT: bool;

    /// A balanced binary `HTree` is a
    const BALANCED: bool;

    // /// Returns the length of a given `HList`
    // #[inline]
    // fn len(&self) -> usize {
    //     Self::LEN
    // }
    //
    // /// Returns whether a given `HList` is empty
    // #[inline]
    // fn is_empty(&self) -> bool {
    //     Self::LEN == 0
    // }
    //
    // /// Prepends an item to the current `HList`
    // #[inline]
    // fn prepend<H>(self, h: H) -> HCons<H, Self> {
    //     HCons {
    //         head: h,
    //         tail: self,
    //     }
    // }
}

/// Represents the outermost extent of a heterogeneous binary tree, i.e. an empty tree
#[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
pub struct HTip;

// impl HTree for HNil {
//     const FULL: bool = false;
// }

/// Represents a binary `HTree` which holds two (possibly empty) child subtrees
#[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
pub struct HFork<T, L, R> {
    pub value: T,
    pub left: L,
    pub right: R,
}

// impl<T, L, R> HTree for HFork<T, L, R> {
//     const FULL: bool = false;
// }

/// A leaf node is a `HFork` with precisely two `HTip` subtrees, i.e. no left or right children.
pub type HLeaf<T> = HFork<T, HTip, HTip>;
