#![allow(
    clippy::same_name_method,
    clippy::inline_always,
    clippy::mismatching_type_param_order,
    clippy::renamed_function_params,
    clippy::too_long_first_doc_paragraph
)]

//! Module that holds `HList` data structures, implementations, and trait.
//! Ripped straight from [frunk](https://github.com/lloydmeta/frunk/) so credit to that crate :)

use crate::data::indices::{Here, Suffixed, There};
use std::ops::Add;

/// Trait for HList-y behaviour
///
/// An `HList` is a heterogeneous list, one that is statically typed at compile time. In simple terms,
/// it is just an arbitrarily-nested Tuple2.
pub trait HList: Sized {
    /// Returns the length of a given `HList` type without making use of any references, or in fact,
    /// any values at all.
    const LEN: usize;

    /// Returns the length of a given `HList`
    #[inline]
    fn len(&self) -> usize {
        Self::LEN
    }

    /// Returns whether a given `HList` is empty
    #[inline]
    fn is_empty(&self) -> bool {
        Self::LEN == 0
    }

    /// Prepends an item to the current `HList`
    #[inline]
    fn prepend<H>(self, h: H) -> HCons<H, Self> {
        HCons {
            head: h,
            tail: self,
        }
    }
}

/// Represents the right-most end of a heterogeneous list
#[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
pub struct HNil;

impl HList for HNil {
    const LEN: usize = 0;
}

/// Represents the most basic non-empty `HList`.
/// Its value is held in `head` while its tail is another `HList`.
#[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
pub struct HCons<H, T> {
    pub head: H,
    pub tail: T,
}

impl<H, T: HList> HList for HCons<H, T> {
    const LEN: usize = 1 + <T as HList>::LEN;
}

impl<H, T> HCons<H, T> {
    /// Returns the head of the list and the tail of the list as a tuple2.
    /// The original list is consumed
    #[inline]
    pub fn pop(self) -> (H, T) {
        (self.head, self.tail)
    }
}

/// Takes an element and an `Hlist` and returns another one with the element prepended to the original list.
/// The original list is consumed
#[inline]
pub const fn h_cons<H, T: HList>(h: H, tail: T) -> HCons<H, T> {
    HCons { head: h, tail }
}

// Inherent methods shared by HNil and HCons.
macro_rules! gen_inherent_methods {
    (impl<$($TyPar:ident),*> $Struct:ty { ... })
    => {
        impl<$($TyPar),*> $Struct {
            /// Returns the length of a given `HList`
            #[inline(always)]
            #[must_use]
            pub fn len(&self) -> usize
            where Self: HList,
            {
                HList::len(self)
            }

            /// Returns whether a given `HList` is empty
            #[inline(always)]
            #[must_use]
            pub fn is_empty(&self) -> bool
            where Self: HList,
            {
                HList::is_empty(self)
            }

            /// Prepend an item to the current `HList`
            #[inline(always)]
            pub fn prepend<H>(self, h: H) -> HCons<H, Self>
            where Self: HList,
            {
                HList::prepend(self, h)
            }

            /// Consume the current `HList` and return an `HList` with the requested shape.
            ///
            /// `sculpt` allows us to extract/reshape/sculpt the current `HList` into another shape,
            /// provided that the requested shape's types are are contained within the current `HList`.
            ///
            /// The `Indices` type parameter allows the compiler to figure out that `Ts` and `Self`
            /// can be morphed into each other.
            #[inline(always)]
            #[must_use]
            pub fn sculpt<Ts, Indices>(self) -> (Ts, <Self as Sculptor<Ts, Indices>>::Remainder)
            where Self: Sculptor<Ts, Indices>,
            {
                Sculptor::sculpt(self)
            }

            // /// Reverse the HList. // TODO: uncomment this after "reverse" trait added
            // #[inline(always)]
            // pub fn into_reverse(self) -> <Self as IntoReverse>::Output
            // where Self: IntoReverse,
            // {
            //     IntoReverse::into_reverse(self)
            // }

            // /// Return an HList where the contents are references to the original HList on which
            // /// this method was called. // TODO: uncomment this after "to ref" is added
            // #[inline(always)]
            // #[allow(clippy::wrong_self_convention)]
            // pub fn to_ref<'a>(&'a self) -> <Self as ToRef<'a>>::Output
            //     where Self: ToRef<'a>,
            // {
            //     ToRef::to_ref(self)
            // }

            // /// Return an `HList` where the contents are mutable references to the original `HList`
            // /// on which this method was called. // TODO: uncomment this after "to mut" is added
            // #[inline(always)]
            // pub fn to_mut<'a>(&'a mut self) -> <Self as ToMut<'a>>::Output
            // where
            //     Self: ToMut<'a>,
            // {
            //     ToMut::to_mut(self)
            // }

            /// Apply a function to each element of an `HList`.
            ///
            /// This transforms some `HList<A, B, C, ..., E>` into some `HList<T, U, V, ..., Z>`.
            /// A variety of types are supported for the folder argument:
            ///
            /// * An `HList` of closures (one for each element).
            /// * A single closure (for mapping an `HList` that is homogenous).
            // /// * A single [`Poly`]. // TODO: add this line back once `Poly` struct is added
            #[inline(always)]
            pub fn map<F>(self, mapper: F) -> <Self as HMappable<F>>::Output
            where Self: HMappable<F>,
            {
                HMappable::map(self, mapper)
            }

            /// Zip two `HLists` together.
            ///
            /// This zips a `HList<A1, B1, ..., C1>` with a `HList<A2, B2, ..., C2>` to make a
            /// `HList<(A1, A2), (B1, B2), ..., (C1, C2)>`
            #[inline(always)]
            pub fn zip<Other>(self, other: Other) -> <Self as HZippable<Other>>::Zipped
            where Self: HZippable<Other>,
            {
                HZippable::zip(self, other)
            }

            /// Perform a left fold over an `HList`.
            ///
            /// This transforms some `HList<A, B, C, ..., E>` into a single value by visiting all
            /// of the elements in left-to-right order. A variety of types are supported for the
            /// mapper argument:
            ///
            /// * An `HList` of closures (one for each element).
            /// * A single closure (for folding an `HList` that is homogenous).
            // /// * A single [`Poly`]. TODO: uncomment once `Poly` struct is added
            ///
            /// The accumulator can freely change type over the course of the call.
            #[inline(always)]
            pub fn foldl<Folder, Acc>(
                self,
                folder: Folder,
                acc: Acc,
            ) -> <Self as HFoldLeftable<Folder, Acc>>::Output
            where Self: HFoldLeftable<Folder, Acc>,
            {
                HFoldLeftable::foldl(self, folder, acc)
            }

            /// Perform a right fold over an `HList`.
            ///
            /// This transforms some `HList<A, B, C, ..., E>` into a single value by visiting all of
            /// the elements in reverse order. A variety of types are supported for the mapper argument:
            ///
            /// * An `HList` of closures (one for each element).
            /// * A single closure (for folding an `HList` that is homogenous),
            ///   taken by reference.
            // /// * A single [`Poly`]. TODO: uncomment once `Poly` struct is added
            ///
            /// The accumulator can freely change type over the course of the call.
            ///
            /// # Comparison to `foldl`
            ///
            /// While the order of element traversal in `foldl` may seem more natural, `foldr` does
            /// have its use cases, in particular when it is used to build something that reflects
            /// the structure of the original `HList` (such as folding an `HList` of `Option`s into
            /// an `Option` of an `HList`). An implementation of such a function using `foldl` will
            /// tend to reverse the list, while `foldr` will tend to preserve its order.
            ///
            /// The reason for this is because `foldr` performs what is known as "structural
            /// induction;" it can be understood as follows:
            ///
            /// * Write out the `HList` in terms of [`h_cons`] and [`HNil`].
            /// * Substitute each [`h_cons`] with a function, and substitute [`HNil`] with `init`
            #[inline(always)]
            pub fn foldr<Folder, Init>(
                self,
                folder: Folder,
                init: Init,
            ) -> <Self as HFoldRightable<Folder, Init>>::Output
            where Self: HFoldRightable<Folder, Init>,
            {
                HFoldRightable::foldr(self, folder, init)
            }

            /// Extend the contents of this `HList` with another `HList`
            ///
            /// This exactly the same as the [`Add`][Add] impl.
            #[allow(clippy::missing_const_for_fn, clippy::arithmetic_side_effects)]
            #[inline(always)]
            pub fn extend<Other>(
                self,
                other: Other
            ) -> <Self as Add<Other>>::Output
            where
                Self: Add<Other>,
                Other: HList,
            {
                self + other
            }
        }
    };
}

gen_inherent_methods! {
    impl<> HNil { ... }
}
gen_inherent_methods! {
    impl<Head, Tail> HCons<Head, Tail> { ... }
}

// HCons-only inherent methods.
impl<Head, Tail> HCons<Head, Tail> {
    /// Borrow an element by type from an `HList`.
    #[inline(always)]
    pub fn get<T, Index>(&self) -> &T
    where
        Self: Selector<T, Index>,
    {
        Selector::get(self)
    }

    /// Mutably borrow an element by type from an `HList`.
    #[inline(always)]
    pub fn get_mut<T, Index>(&mut self) -> &mut T
    where
        Self: Selector<T, Index>,
    {
        Selector::get_mut(self)
    }

    /// Remove an element by type from an `HList`.
    ///
    /// The remaining elements are returned along with it.
    #[inline(always)]
    pub fn pluck<T, Index>(self) -> (T, <Self as Plucker<T, Index>>::Remainder)
    where
        Self: Plucker<T, Index>,
    {
        Plucker::pluck(self)
    }

    /// Turns an `HList` into nested Tuple2s, which are less troublesome to pattern match and have
    /// a nicer type signature.
    #[inline(always)]
    pub fn into_tuple2(
        self,
    ) -> (
        <Self as IntoTuple2>::HeadType,
        <Self as IntoTuple2>::TailOutput,
    )
    where
        Self: IntoTuple2,
    {
        IntoTuple2::into_tuple2(self)
    }
}

impl<RHS> Add<RHS> for HNil
where
    RHS: HList,
{
    type Output = RHS;

    #[inline(always)]
    fn add(self, rhs: RHS) -> RHS {
        rhs
    }
}

impl<H, T, RHS> Add<RHS> for HCons<H, T>
where
    T: Add<RHS>,
    RHS: HList,
{
    type Output = HCons<H, <T as Add<RHS>>::Output>;

    #[allow(clippy::arithmetic_side_effects)]
    #[inline(always)]
    fn add(self, rhs: RHS) -> Self::Output {
        HCons {
            head: self.head,
            tail: self.tail + rhs,
        }
    }
}

/// Trait for borrowing an `HList` element by type
///
/// This trait is part of the implementation of the inherent method [`HCons::get`].
/// Please see that method for more information.
///
/// You only need to import this trait when working with generic `HLists` of unknown type.
/// If you have an `HList` of known type, then `list.get()` should "just work" even without the trait.
pub trait Selector<S, I> {
    /// Borrow an element by type from an `HList`.
    ///
    /// Please see the [inherent method] for more information.
    ///
    /// The only difference between that inherent method and this trait method is the location of
    /// the type parameters (here, they are on the trait rather than the method).
    fn get(&self) -> &S;

    /// Mutably borrow an element by type from an `HList`.
    ///
    /// Please see the [inherent method] for more information.
    ///
    /// The only difference between that inherent method and this trait method is the location of
    /// the type parameters (here, they are on the trait rather than the method).
    fn get_mut(&mut self) -> &mut S;
}

impl<T, Tail> Selector<T, Here> for HCons<T, Tail> {
    #[inline(always)]
    fn get(&self) -> &T {
        &self.head
    }

    #[inline(always)]
    fn get_mut(&mut self) -> &mut T {
        &mut self.head
    }
}

impl<Head, Tail, FromTail, TailIndex> Selector<FromTail, There<TailIndex>> for HCons<Head, Tail>
where
    Tail: Selector<FromTail, TailIndex>,
{
    #[inline(always)]
    fn get(&self) -> &FromTail {
        self.tail.get()
    }

    #[inline(always)]
    fn get_mut(&mut self) -> &mut FromTail {
        self.tail.get_mut()
    }
}

/// Trait defining extraction from a given `HList`
///
/// This trait is part of the implementation of the inherent method [`HCons::pluck`].
/// Please see that method for more information.
///
/// You only need to import this trait when working with generic `HLists` of unknown type.
/// If you have an `HList` of known type, then `list.pluck()` should "just work" even without the trait.
pub trait Plucker<Target, Index> {
    /// What is left after you pluck the target from the Self
    type Remainder;

    /// Remove an element by type from an `HList`.
    ///
    /// Please see the [inherent method] for more information.
    ///
    /// The only difference between that inherent method and this trait method is the location of
    /// the type parameters. (here, they are on the trait rather than the method)
    fn pluck(self) -> (Target, Self::Remainder);
}

/// Implementation when the pluck target is in head
impl<T, Tail> Plucker<T, Here> for HCons<T, Tail> {
    type Remainder = Tail;

    #[inline(always)]
    fn pluck(self) -> (T, Self::Remainder) {
        (self.head, self.tail)
    }
}

/// Implementation when the pluck target is in the tail
impl<Head, Tail, FromTail, TailIndex> Plucker<FromTail, There<TailIndex>> for HCons<Head, Tail>
where
    Tail: Plucker<FromTail, TailIndex>,
{
    type Remainder = HCons<Head, <Tail as Plucker<FromTail, TailIndex>>::Remainder>;

    #[inline(always)]
    fn pluck(self) -> (FromTail, Self::Remainder) {
        let (target, tail_remainder): (
            FromTail,
            <Tail as Plucker<FromTail, TailIndex>>::Remainder,
        ) = <Tail as Plucker<FromTail, TailIndex>>::pluck(self.tail);
        (
            target,
            HCons {
                head: self.head,
                tail: tail_remainder,
            },
        )
    }
}

// TODO: uncomment once `ToRef` added back
// /// Implementation when target is reference and  the pluck target is in head
// impl<'a, T, Tail: ToRef<'a>> Plucker<&'a T, Here> for &'a HCons<T, Tail> {
//     type Remainder = <Tail as ToRef<'a>>::Output;
//
//     fn pluck(self) -> (&'a T, Self::Remainder) {
//         (&self.head, self.tail.to_ref())
//     }
// }

/// Implementation when target is reference the pluck target is in the tail
impl<'a, Head, Tail, FromTail, TailIndex> Plucker<&'a FromTail, There<TailIndex>>
    for &'a HCons<Head, Tail>
where
    &'a Tail: Plucker<&'a FromTail, TailIndex>,
{
    type Remainder = HCons<&'a Head, <&'a Tail as Plucker<&'a FromTail, TailIndex>>::Remainder>;

    #[inline(always)]
    fn pluck(self) -> (&'a FromTail, Self::Remainder) {
        let (target, tail_remainder): (
            &'a FromTail,
            <&'a Tail as Plucker<&'a FromTail, TailIndex>>::Remainder,
        ) = <&'a Tail as Plucker<&'a FromTail, TailIndex>>::pluck(&self.tail);
        (
            target,
            HCons {
                head: &self.head,
                tail: tail_remainder,
            },
        )
    }
}

/// Trait for pulling out some subset of an `HList`, using type inference.
///
/// This trait is part of the implementation of the inherent method [`HCons::sculpt`].
/// Please see that method for more information.
///
/// You only need to import this trait when working with generic `HLists` of unknown type.
/// If you have an `HList` of known type, then `list.sculpt()` should "just work" even without the trait.
pub trait Sculptor<Target, Indices> {
    type Remainder;

    /// Consumes the current `HList` and returns an `HList` with the requested shape.
    ///
    /// Please see the [inherent method] for more information.
    ///
    /// The only difference between that inherent method and this trait method is the location of
    /// the type parameters. (here, they are on the trait rather than the method)
    fn sculpt(self) -> (Target, Self::Remainder);
}

/// Implementation for when the target is an empty [`HList`] (`HNil`)
///
/// Index type is `HNil` because we don't need an index for finding `HNil`
impl<Source> Sculptor<HNil, HNil> for Source {
    type Remainder = Source;

    #[inline(always)]
    fn sculpt(self) -> (HNil, Self::Remainder) {
        (HNil, self)
    }
}

/// Implementation for when we have a non-empty `HCons` target
///
/// Indices is `HCons<IndexHead, IndexTail>` here because the compiler is being asked to figure out
/// the Index for Plucking the first item of type `THead` out of Self and the rest (`IndexTail`) is
/// for the plucker's remainder induce.
impl<THead, TTail, SHead, STail, IndexHead, IndexTail>
    Sculptor<HCons<THead, TTail>, HCons<IndexHead, IndexTail>> for HCons<SHead, STail>
where
    Self: Plucker<THead, IndexHead>,
    <Self as Plucker<THead, IndexHead>>::Remainder: Sculptor<TTail, IndexTail>,
{
    type Remainder =
        <<Self as Plucker<THead, IndexHead>>::Remainder as Sculptor<TTail, IndexTail>>::Remainder;

    #[inline(always)]
    fn sculpt(self) -> (HCons<THead, TTail>, Self::Remainder) {
        let (p, r): (THead, <Self as Plucker<THead, IndexHead>>::Remainder) = self.pluck();
        let (tail, tail_remainder): (TTail, Self::Remainder) = r.sculpt();
        (HCons { head: p, tail }, tail_remainder)
    }
}

// TODO: uncomment once "into-reverse" trait is added back
// impl IntoReverse for HNil {
//     type Output = HNil;
//     fn into_reverse(self) -> Self::Output {
//         self
//     }
// }
//
// impl<H, Tail> IntoReverse for HCons<H, Tail>
// where
//     Tail: IntoReverse,
//     <Tail as IntoReverse>::Output: Add<HCons<H, HNil>>,
// {
//     type Output = <<Tail as IntoReverse>::Output as Add<HCons<H, HNil>>>::Output;
//
//     fn into_reverse(self) -> Self::Output {
//         self.tail.into_reverse()
//             + HCons {
//                 head: self.head,
//                 tail: HNil,
//             }
//     }
// }

// TODO: reuncomment once `Poly` is added back
// impl<P, H, Tail> HMappable<Poly<P>> for HCons<H, Tail>
// where
//     P: Func<H>,
//     Tail: HMappable<Poly<P>>,
// {
//     type Output = HCons<<P as Func<H>>::Output, <Tail as HMappable<Poly<P>>>::Output>;
//     fn map(self, poly: Poly<P>) -> Self::Output {
//         HCons {
//             head: P::call(self.head),
//             tail: self.tail.map(poly),
//         }
//     }
// }

/// Trait for mapping over an `HList`
///
/// This trait is part of the implementation of the inherent method [`HCons::map`].
/// Please see that method for more information.
///
/// You only need to import this trait when working with generic `HLists` or Mappers of unknown type.
/// If the type of everything is known, then `list.map(f)` should "just work" even without the trait.
pub trait HMappable<Mapper> {
    type Output;

    /// Apply a function to each element of an `HList`.
    ///
    /// Please see the [inherent method] for more information.
    ///
    /// The only difference between that inherent method and this trait method is the location of
    /// the type parameters. (here, they are on the trait rather than the method)
    fn map(self, mapper: Mapper) -> Self::Output;
}

impl<F> HMappable<F> for HNil {
    type Output = Self;

    #[inline(always)]
    fn map(self, _: F) -> Self::Output {
        Self
    }
}

impl<F, R, H, Tail> HMappable<F> for HCons<H, Tail>
where
    F: Fn(H) -> R,
    Tail: HMappable<F>,
{
    type Output = HCons<R, <Tail as HMappable<F>>::Output>;

    #[inline]
    fn map(self, f: F) -> Self::Output {
        let Self { head, tail } = self;
        HCons {
            head: f(head),
            tail: tail.map(f),
        }
    }
}

impl<F, R, MapperTail, H, Tail> HMappable<HCons<F, MapperTail>> for HCons<H, Tail>
where
    F: FnOnce(H) -> R,
    Tail: HMappable<MapperTail>,
{
    type Output = HCons<R, <Tail as HMappable<MapperTail>>::Output>;

    #[inline]
    fn map(self, mapper: HCons<F, MapperTail>) -> Self::Output {
        let Self { head, tail } = self;
        HCons {
            head: (mapper.head)(head),
            tail: tail.map(mapper.tail),
        }
    }
}

/// Trait for zipping `HLists`
///
/// This trait is part of the implementation of the inherent method [`HCons::zip`]. Please see that
/// method for more information.
///
/// You only need to import this trait when working with generic `HLists` of unknown type.
/// If the type of everything is known, then `list.zip(list2)` should "just work" even without the trait.
pub trait HZippable<Other> {
    type Zipped: HList;

    /// Zip this `HList` with another one.
    ///
    /// Please see the [inherent method] for more information.
    fn zip(self, other: Other) -> Self::Zipped;
}

impl HZippable<Self> for HNil {
    type Zipped = Self;
    #[inline]
    fn zip(self, _other: Self) -> Self::Zipped {
        Self
    }
}

impl<H1, T1, H2, T2> HZippable<HCons<H2, T2>> for HCons<H1, T1>
where
    T1: HZippable<T2>,
{
    type Zipped = HCons<(H1, H2), T1::Zipped>;
    #[inline]
    fn zip(self, other: HCons<H2, T2>) -> Self::Zipped {
        HCons {
            head: (self.head, other.head),
            tail: self.tail.zip(other.tail),
        }
    }
}

/// Trait for performing a right fold over an `HList`
///
/// This trait is part of the implementation of the inherent method [`HCons::foldr`]. Please see
/// that method for more information.
///
/// You only need to import this trait when working with generic `HLists` or Folders of unknown type.
/// If the type of everything is known, then `list.foldr(f, init)` should "just work" even without the trait.
pub trait HFoldRightable<Folder, Init> {
    type Output;

    /// Perform a right fold over an `HList`.
    ///
    /// Please see the [inherent method] for more information.
    ///
    /// The only difference between that inherent method and this trait method is the location of
    /// the type parameters. (here, they are on the trait rather than the method)
    fn foldr(self, folder: Folder, i: Init) -> Self::Output;
}

impl<F, Init> HFoldRightable<F, Init> for HNil {
    type Output = Init;
    #[inline]
    fn foldr(self, _: F, i: Init) -> Self::Output {
        i
    }
}

impl<F, FolderHeadR, FolderTail, H, Tail, Init> HFoldRightable<HCons<F, FolderTail>, Init>
    for HCons<H, Tail>
where
    Tail: HFoldRightable<FolderTail, Init>,
    F: FnOnce(<Tail as HFoldRightable<FolderTail, Init>>::Output, H) -> FolderHeadR,
{
    type Output = FolderHeadR;
    #[inline]
    fn foldr(self, folder: HCons<F, FolderTail>, init: Init) -> Self::Output {
        let folded_tail = self.tail.foldr(folder.tail, init);
        (folder.head)(folded_tail, self.head)
    }
}

impl<F, R, H, Tail, Init> HFoldRightable<F, Init> for HCons<H, Tail>
where
    Tail: foldr_owned::HFoldRightableOwned<F, Init>,
    F: Fn(<Tail as HFoldRightable<F, Init>>::Output, H) -> R,
{
    type Output = R;
    #[inline]
    fn foldr(self, folder: F, init: Init) -> Self::Output {
        foldr_owned::HFoldRightableOwned::real_foldr(self, folder, init).0
    }
}

/// [`HFoldRightable`] inner mechanics for folding with a folder that needs to be owned.
pub mod foldr_owned {
    use super::{HCons, HFoldRightable, HNil};

    /// A real `foldr` for the folder that must be owned to fold.
    ///
    /// Due to `HList` being a recursive struct and not linear array, the only way to fold it is
    /// recursive.
    ///
    /// However, there are differences in the `foldl` and `foldr` traversing the `HList`:
    ///
    /// 1. `foldl` calls `folder(head)` and then passes the ownership of the folder to the next
    ///    recursive call.
    /// 2. `foldr` passes the ownership of the folder to the next recursive call, and then tries to
    ///    call `folder(head)`; but the ownership is already gone!
    pub trait HFoldRightableOwned<Folder, Init>: HFoldRightable<Folder, Init> {
        fn real_foldr(self, folder: Folder, init: Init) -> (Self::Output, Folder);
    }

    impl<F, Init> HFoldRightableOwned<F, Init> for HNil {
        #[inline]
        fn real_foldr(self, f: F, i: Init) -> (Self::Output, F) {
            (i, f)
        }
    }

    impl<F, H, Tail, Init> HFoldRightableOwned<F, Init> for HCons<H, Tail>
    where
        Self: HFoldRightable<F, Init>,
        Tail: HFoldRightableOwned<F, Init>,
        F: Fn(<Tail as HFoldRightable<F, Init>>::Output, H) -> Self::Output,
    {
        #[inline]
        fn real_foldr(self, folder: F, init: Init) -> (Self::Output, F) {
            let (folded_tail, folder) = self.tail.real_foldr(folder, init);
            (folder(folded_tail, self.head), folder)
        }
    }
}

// TODO: add back when `Poly` is added back
// impl<P, R, H, Tail, Init> HFoldRightable<Poly<P>, Init> for HCons<H, Tail>
// where
//     Tail: HFoldRightable<Poly<P>, Init>,
//     P: Func<(<Tail as HFoldRightable<Poly<P>, Init>>::Output, H), Output = R>,
// {
//     type Output = R;
//
//     fn foldr(self, poly: Poly<P>, init: Init) -> Self::Output {
//         let HCons { head, tail } = self;
//         let folded_tail = tail.foldr(poly, init);
//         P::call((folded_tail, head))
//     }
// }

// TODO: add back when `ToRef` is added back
// impl<'a> ToRef<'a> for HNil {
//     type Output = HNil;
//
//     #[inline(always)]
//     fn to_ref(&'a self) -> Self::Output {
//         HNil
//     }
// }
//
// impl<'a, H, Tail> ToRef<'a> for HCons<H, Tail>
// where
//     H: 'a,
//     Tail: ToRef<'a>,
// {
//     type Output = HCons<&'a H, <Tail as ToRef<'a>>::Output>;
//
//     #[inline(always)]
//     fn to_ref(&'a self) -> Self::Output {
//         HCons {
//             head: &self.head,
//             tail: self.tail.to_ref(),
//         }
//     }
// }

// TODO: add back when `ToMut` is added back
// impl<'a> ToMut<'a> for HNil {
//     type Output = HNil;
//
//     #[inline(always)]
//     fn to_mut(&'a mut self) -> Self::Output {
//         HNil
//     }
// }
//
// impl<'a, H, Tail> ToMut<'a> for HCons<H, Tail>
// where
//     H: 'a,
//     Tail: ToMut<'a>,
// {
//     type Output = HCons<&'a mut H, <Tail as ToMut<'a>>::Output>;
//
//     #[inline(always)]
//     fn to_mut(&'a mut self) -> Self::Output {
//         HCons {
//             head: &mut self.head,
//             tail: self.tail.to_mut(),
//         }
//     }
// }

/// Trait for performing a left fold over an `HList`
///
/// This trait is part of the implementation of the inherent method [`HCons::foldl`].
/// Please see that method for more information.
///
/// You only need to import this trait when working with generic `HLists` or Mappers of unknown type.
/// If the type of everything is known, then `list.foldl(f, acc)` should "just work" even without the trait.
pub trait HFoldLeftable<Folder, Acc> {
    type Output;

    /// Perform a left fold over an `HList`.
    ///
    /// Please see the [inherent method] for more information.
    ///
    /// The only difference between that inherent method and this trait method is the location of
    /// the type parameters. (here, they are on the trait rather than the method)
    fn foldl(self, folder: Folder, acc: Acc) -> Self::Output;
}

impl<F, Acc> HFoldLeftable<F, Acc> for HNil {
    type Output = Acc;
    #[inline]
    fn foldl(self, _: F, acc: Acc) -> Self::Output {
        acc
    }
}

impl<F, R, FTail, H, Tail, Acc> HFoldLeftable<HCons<F, FTail>, Acc> for HCons<H, Tail>
where
    Tail: HFoldLeftable<FTail, R>,
    F: FnOnce(Acc, H) -> R,
{
    type Output = <Tail as HFoldLeftable<FTail, R>>::Output;
    #[inline]
    fn foldl(self, folder: HCons<F, FTail>, acc: Acc) -> Self::Output {
        let Self { head, tail } = self;
        tail.foldl(folder.tail, (folder.head)(acc, head))
    }
}

// TODO: add back when `Poly` is added back
// impl<P, R, H, Tail, Acc> HFoldLeftable<Poly<P>, Acc> for HCons<H, Tail>
// where
//     Tail: HFoldLeftable<Poly<P>, R>,
//     P: Func<(Acc, H), Output = R>,
// {
//     type Output = <Tail as HFoldLeftable<Poly<P>, R>>::Output;
//
//     fn foldl(self, poly: Poly<P>, acc: Acc) -> Self::Output {
//         let HCons { head, tail } = self;
//         let r = P::call((acc, head));
//         tail.foldl(poly, r)
//     }
// }

/// Implementation for folding over an `HList` using a single function that can handle all cases
impl<F, H, Tail, Acc> HFoldLeftable<F, Acc> for HCons<H, Tail>
where
    Tail: HFoldLeftable<F, Acc>,
    F: Fn(Acc, H) -> Acc,
{
    type Output = <Tail as HFoldLeftable<F, Acc>>::Output;
    #[inline]
    fn foldl(self, f: F, acc: Acc) -> Self::Output {
        let Self { head, tail } = self;
        let acc = f(acc, head);
        tail.foldl(f, acc)
    }
}

/// Trait for transforming an `HList` into a nested tuple.
///
/// This trait is part of the implementation of the inherent method [`HCons::into_tuple2`].
/// Please see that method for more information.
///
/// This operation is not useful in generic contexts, so it is unlikely that you should ever need
/// to import this trait. Do not worry; if you have an `HList` of known type, then `list.into_tuple2()`
/// should "just work," even without the trait.
pub trait IntoTuple2 {
    /// The 0 element in the output tuple
    type HeadType;

    /// The 1 element in the output tuple
    type TailOutput;

    /// Turns an `HList` into nested Tuple2s, which are less troublesome to pattern match
    /// and have a nicer type signature.
    ///
    /// Please see the [inherent method] for more information.
    fn into_tuple2(self) -> (Self::HeadType, Self::TailOutput);
}

impl<T1, T2> IntoTuple2 for HCons<T1, HCons<T2, HNil>> {
    type HeadType = T1;
    type TailOutput = T2;
    #[inline]
    fn into_tuple2(self) -> (Self::HeadType, Self::TailOutput) {
        (self.head, self.tail.head)
    }
}

impl<T, Tail> IntoTuple2 for HCons<T, Tail>
where
    Tail: IntoTuple2,
{
    type HeadType = T;
    type TailOutput = (
        <Tail as IntoTuple2>::HeadType,
        <Tail as IntoTuple2>::TailOutput,
    );
    #[inline]
    fn into_tuple2(self) -> (Self::HeadType, Self::TailOutput) {
        (self.head, self.tail.into_tuple2())
    }
}

#[allow(clippy::from_over_into)]
impl<H, Tail> Into<Vec<H>> for HCons<H, Tail>
where
    Tail: Into<Vec<H>> + HList,
{
    #[inline]
    fn into(self) -> Vec<H> {
        let h = self.head;
        let t = self.tail;
        let mut v = Vec::with_capacity(<Self as HList>::LEN);
        v.push(h);
        let mut t_vec: Vec<H> = t.into();
        v.append(&mut t_vec);
        v
    }
}
#[allow(clippy::from_over_into)]
impl<T> Into<Vec<T>> for HNil {
    #[inline]
    fn into(self) -> Vec<T> {
        Vec::with_capacity(0)
    }
}

impl Default for HNil {
    #[inline]
    fn default() -> Self {
        Self
    }
}

impl<T: Default, Tail: Default + HList> Default for HCons<T, Tail> {
    #[inline]
    fn default() -> Self {
        h_cons(T::default(), Tail::default())
    }
}

/// Indexed type conversions of `T -> Self` with index `I`. This is a generalized version
/// of `From` which for example allows the caller to use default values for parts of `Self`
/// and thus "fill in the blanks".
///
/// `LiftFrom` is the reciprocal of `LiftInto`.
pub trait LiftFrom<T, I> {
    /// Performs the indexed conversion.
    fn lift_from(part: T) -> Self;
}

/// Free function version of `LiftFrom::lift_from`.
#[inline]
pub fn lift_from<I, T, PF: LiftFrom<T, I>>(part: T) -> PF {
    PF::lift_from(part)
}

/// An indexed conversion that consumes `self`, and produces a `T`. To produce `T`, the index `I`
/// may be used to for example "fill in the blanks".
///
/// `LiftInto` is the reciprocal of `LiftFrom`.
pub trait LiftInto<T, I> {
    /// Performs the indexed conversion.
    fn lift_into(self) -> T;
}

impl<T, U, I> LiftInto<U, I> for T
where
    U: LiftFrom<T, I>,
{
    #[inline]
    fn lift_into(self) -> U {
        LiftFrom::lift_from(self)
    }
}

impl<T, Tail> LiftFrom<T, Here> for HCons<T, Tail>
where
    Tail: Default + HList,
{
    #[inline]
    fn lift_from(part: T) -> Self {
        h_cons(part, Tail::default())
    }
}

impl<Head, Tail, ValAtIx, TailIx> LiftFrom<ValAtIx, There<TailIx>> for HCons<Head, Tail>
where
    Head: Default,
    Tail: HList + LiftFrom<ValAtIx, TailIx>,
{
    #[inline]
    fn lift_from(part: ValAtIx) -> Self {
        h_cons(Head::default(), Tail::lift_from(part))
    }
}

impl<Prefix, Suffix> LiftFrom<Prefix, Suffixed<Suffix>> for <Prefix as Add<Suffix>>::Output
where
    Prefix: HList + Add<Suffix>,
    Suffix: Default,
{
    #[allow(clippy::arithmetic_side_effects)]
    #[inline]
    fn lift_from(part: Prefix) -> Self {
        part + Suffix::default()
    }
}
