pub mod alias {
    use crate::family_pattern::ty_constructor::{
        ConstructableTy1, ConstructableTy2, ConstructableTy3, TyConstructor1, TyConstructor2,
        TyConstructor3,
    };

    // type aliases
    pub type TC1<T, A> = <T as TyConstructor1>::TC1<A>;
    pub type TC2<T, A, B> = <T as TyConstructor2>::TC2<A, B>;
    pub type TC3<T, A, B, C> = <T as TyConstructor3>::TC3<A, B, C>;

    pub type CT1<T> = <T as ConstructableTy1>::Constructor;
    pub type CT2<T> = <T as ConstructableTy2>::Constructor;
    pub type CT3<T> = <T as ConstructableTy3>::Constructor;

    // trait aliases
    pub trait ConstructableTySyntax1<T: TyConstructor1, A> =
        ConstructableTy1<Constructor = T, GenericParameter1 = A>;
    pub trait ConstructableTySyntax2<T: TyConstructor2, A, B> =
        ConstructableTy2<Constructor = T, GenericParameter1 = A, GenericParameter2 = B>;
    pub trait ConstructableTySyntax3<T: TyConstructor3, A, B, C> = ConstructableTy3<
            Constructor = T,
            GenericParameter1 = A,
            GenericParameter2 = B,
            GenericParameter3 = C,
        >;
}

// TODO: In this post `https://github.com/rust-lang/rfcs/issues/2190#issuecomment-2678925895` I propose a pattern that uses
//       hypothetical associated-trait feature to solve problem of different GAT-bounds:
//       ```rust
//       trait Family { trait Bounds = UnitTrait; type Of<T: Self::Bounds>; }
//       struct OptionFamily;
//       impl Family for OptionFamily { trait Bounds = Sized; type Of<T> = Option<T>; }
//       //--
//       enum Maybe<T: Clone> { Nothing, Just(T) }
//       struct MaybeFamily;
//       impl Family for OptionFamily { trait Bounds = Clone; type Of<T: Clone> = Maybe<T>; }
//       ```
//       Something __approximating__ this can be achieved with type-witnesses of trait-bounds
//       ```rust
//       trait Family { type Witness<T>: TypeWitness; type Of<T>; }
//       struct OptionFamily;
//       impl Family for OptionFamily {
//         type Witness<T> = SizedWit<T>;
//         type Of<T> = Option<With<T, SizedWit<T>>>;
//       } // `With<T, Self::Witness<T>>` (should....) implement `Sized`??, although _specifically_ for
//         // marker-traits, it may be a bit of a challenge to get those kinds of things to "translate" :((
//       //--
//       enum Maybe<T: Clone> { Nothing, Just(T) }
//       struct MaybeFamily;
//       impl Family for OptionFamily {
//         trait Witness<T> = CloneWit<T>;
//         type Of<T> = Maybe<With<T, CloneWit<T>>>;
//       } // `With<T, CloneWit<T>>` _should_ implement `Clone`
//       ```
//       however clearly this is rather verbose and heavy-handed. We _could_ just introduce
//       newtype constructors which hold those proof-witnesses for every such type-constraint e.g.
//       `CloneVec<T>(Vec<T>, CloneWit<T>)`, but that would be but that may not scale well....
//       ultimately it is something that has to be figured out how to be handled...
// TODO: Figure out proper "partial-type-application" handling, so that this is more Haskell-like
//       to enable more Haskell-like patterns like partially applying data constructor variables
//       i.e. `data Foo f a c = Foo (f a) c` where has kind `f :: * -> * -> *` - this is USEFUL!!

// One-generic type constructors, i.e. `* -> *` types ----------------------------------------------

/// A trait for marker-types that represent type-constructors of concrete [`ConstructableTy1`] types.
/// For example if we have a generic type `Foo<T>` then  it should implement [`ConstructableTy1`],
/// and a marker-type e.g. `FooConstructor` should implement this trait.
#[allow(clippy::too_long_first_doc_paragraph)]
pub trait TyConstructor1 {
    /// An encoding of the type-constructor function, as a generic associated type. Continuing from
    /// the `FooConstructor` example, its trait implementation should be `TC1<A> = Foo<A>`.
    type TC1<A>: ConstructableTy1<Constructor = Self, GenericParameter1 = A>;
}

/// A helper-trait for quantifying over all types that [`TyConstructor1`] can construct, within the
/// context of stipulating [bounds](https://doc.rust-lang.org/rust-by-example/generics/bounds.html)
/// on all constructed types.
///
/// This workaround is needed because Rust's [HRBTs](https://doc.rust-lang.org/nomicon/hrtb.html)
/// only support quantifying over lifetime types for now. This means that writing e.g.
/// `where for<A> Self::TC1<A>: Clone` is not possible, as the generic type `A` is not a lifetime.
#[allow(clippy::too_long_first_doc_paragraph)]
pub trait ForAllConstructedTy1<A>: TyConstructor1 {
    /// The type that [`TyConstructor1`] constructed.
    type ConstructedType = Self::TC1<A>;
}

/// A trait for types that admit generic parameters, and thus are constructable by an associated
/// [`TyConstructor1`].
/// For example if we have a generic type `Foo<T>` then it should implement this trait, and a
/// marker-type e.g. `FooConstructor` should implement [`TyConstructor1`].
#[allow(clippy::too_long_first_doc_paragraph)]
pub trait ConstructableTy1 {
    /// The first positional generic type parameter admitted by this type.
    type GenericParameter1;

    /// The associated [`TyConstructor1`] of this
    type Constructor: ForAllConstructedTy1<Self::GenericParameter1, ConstructedType = Self>;
}

/// An extension trait for [`ConstructableTy1`] types that allows them to be "reified" into the
/// types constructed by their associated [`TyConstructor1`]s, because Rust's type inference is not
/// robust enough to deduce that they are one-and-the-same type.
#[allow(clippy::too_long_first_doc_paragraph)]
#[const_trait]
pub trait ConstructableTyExt1: ConstructableTy1 {
    /// A function for "reifying" [`ConstructableTy1`] into the types constructed by their associated
    /// [`TyConstructor1`]s, because Rust's type inference is not robust enough to deduce that they
    /// are one-and-the-same type.
    fn reify1(self) -> <Self::Constructor as TyConstructor1>::TC1<Self::GenericParameter1>
    where
        Self: Sized;
}

// Two-generic type constructors, i.e. `* -> * -> *` types -----------------------------------------

/// A trait for marker-types that represent type-constructors of concrete [`ConstructableTy2`] types.
/// For example if we have a generic type `Foo<T,F>` then  it should implement [`ConstructableTy2`],
/// and a marker-type e.g. `FooConstructor` should implement this trait.
#[allow(clippy::too_long_first_doc_paragraph)]
pub trait TyConstructor2 {
    /// An encoding of the type-constructor function, as a generic associated type. Continuing from
    /// the `FooConstructor` example, its trait implementation should be `TC2<A,B> = Foo<A,B>`.
    type TC2<A, B>: ConstructableTy2<Constructor = Self, GenericParameter1 = A, GenericParameter2 = B>;
}

/// A helper-trait for quantifying over all types that [`TyConstructor2`] can construct, within the
/// context of stipulating [bounds](https://doc.rust-lang.org/rust-by-example/generics/bounds.html)
/// on all constructed types.
///
/// This workaround is needed because Rust's [HRBTs](https://doc.rust-lang.org/nomicon/hrtb.html)
/// only support quantifying over lifetime types for now. This means that writing e.g.
/// `where for<A,B> Self::TC2<A,B>: Clone` is not possible, as the generic types `A` and `B` are not
/// lifetimes.
#[allow(clippy::too_long_first_doc_paragraph)]
pub trait ForAllConstructedTy2<A, B>: TyConstructor2 {
    /// The type that [`TyConstructor2`] constructed.
    type ConstructedType = Self::TC2<A, B>;
}

/// A trait for types that admit generic parameters, and thus are constructable by an associated
/// [`TyConstructor2`].
/// For example if we have a generic type `Foo<T,F>` then it should implement this trait, and a
/// marker-type e.g. `FooConstructor` should implement [`TyConstructor2`].
#[allow(clippy::too_long_first_doc_paragraph)]
pub trait ConstructableTy2 {
    /// The first positional generic type parameter admitted by this type.
    type GenericParameter1;
    /// The second positional generic type parameter admitted by this type.
    type GenericParameter2;

    /// The associated [`TyConstructor2`] of this
    type Constructor: ForAllConstructedTy2<
            Self::GenericParameter1,
            Self::GenericParameter2,
            ConstructedType = Self,
        >;
}

/// An extension trait for [`ConstructableTy2`] types that allows them to be "reified" into the
/// types constructed by their associated [`TyConstructor2`]s, because Rust's type inference is not
/// robust enough to deduce that they are one-and-the-same type.
#[allow(clippy::too_long_first_doc_paragraph)]
#[const_trait]
pub trait ConstructableTyExt2: ConstructableTy2 {
    /// A function for "reifying" [`ConstructableTy2`] into the types constructed by their associated
    /// [`TyConstructor2`]s, because Rust's type inference is not robust enough to deduce that they
    /// are one-and-the-same type.
    fn reify2(
        self,
    ) -> <Self::Constructor as TyConstructor2>::TC2<Self::GenericParameter1, Self::GenericParameter2>
    where
        Self: Sized;
}

// Three-generic type constructors, i.e. `* -> * -> * -> *` types ----------------------------------

/// A trait for marker-types that represent type-constructors of concrete [`ConstructableTy3`] types.
/// For example if we have a generic type `Foo<T,F,U>` then  it should implement [`ConstructableTy3`],
/// and a marker-type e.g. `FooConstructor` should implement this trait.
#[allow(clippy::too_long_first_doc_paragraph)]
pub trait TyConstructor3 {
    /// An encoding of the type-constructor function, as a generic associated type. Continuing from
    /// the `FooConstructor` example, its trait implementation should be `TC3<A,B,C> = Foo<A,B,C>`.
    type TC3<A, B, C>: ConstructableTy3<
            Constructor = Self,
            GenericParameter1 = A,
            GenericParameter2 = B,
            GenericParameter3 = C,
        >;
}

/// A helper-trait for quantifying over all types that [`TyConstructor3`] can construct, within the
/// context of stipulating [bounds](https://doc.rust-lang.org/rust-by-example/generics/bounds.html)
/// on all constructed types.
///
/// This workaround is needed because Rust's [HRBTs](https://doc.rust-lang.org/nomicon/hrtb.html)
/// only support quantifying over lifetime types for now. This means that writing e.g.
/// `where for<A,B,C> Self::TC3<A,B,C>: Clone` is not possible, as the generic types `A`, `B` and `C`
/// are not lifetimes.
#[allow(clippy::too_long_first_doc_paragraph)]
pub trait ForAllConstructedTy3<A, B, C>: TyConstructor3 {
    /// The type that [`TyConstructor3`] constructed.
    type ConstructedType = Self::TC3<A, B, C>;
}

/// A trait for types that admit generic parameters, and thus are constructable by an associated
/// [`TyConstructor3`].
/// For example if we have a generic type `Foo<T,F,U>` then it should implement this trait, and a
/// marker-type e.g. `FooConstructor` should implement [`TyConstructor3`].
#[allow(clippy::too_long_first_doc_paragraph)]
pub trait ConstructableTy3 {
    /// The first positional generic type parameter admitted by this type.
    type GenericParameter1;
    /// The second positional generic type parameter admitted by this type.
    type GenericParameter2;
    /// The third positional generic type parameter admitted by this type.
    type GenericParameter3;

    /// The associated [`TyConstructor3`] of this
    type Constructor: ForAllConstructedTy3<
            Self::GenericParameter1,
            Self::GenericParameter2,
            Self::GenericParameter3,
            ConstructedType = Self,
        >;
}

/// An extension trait for [`ConstructableTy3`] types that allows them to be "reified" into the
/// types constructed by their associated [`TyConstructor3`]s, because Rust's type inference is not
/// robust enough to deduce that they are one-and-the-same type.
#[allow(clippy::too_long_first_doc_paragraph)]
#[const_trait]
pub trait ConstructableTyExt3: ConstructableTy3 {
    /// A function for "reifying" [`ConstructableTy3`] into the types constructed by their associated
    /// [`TyConstructor3`]s, because Rust's type inference is not robust enough to deduce that they
    /// are one-and-the-same type.
    fn reify3(
        self,
    ) -> <Self::Constructor as TyConstructor3>::TC3<
        Self::GenericParameter1,
        Self::GenericParameter2,
        Self::GenericParameter3,
    >
    where
        Self: Sized;
}

mod impls {
    #![allow(clippy::inline_always, clippy::single_call_fn)]

    use crate::family_pattern::ty_constructor::{
        ConstructableTy1, ConstructableTy2, ConstructableTy3, ConstructableTyExt1,
        ConstructableTyExt2, ConstructableTyExt3, ForAllConstructedTy1, ForAllConstructedTy2,
        ForAllConstructedTy3, TyConstructor1, TyConstructor2, TyConstructor3,
    };

    // blanket `ForAllConstructedTy*` implementations
    impl<T: TyConstructor1, A> ForAllConstructedTy1<A> for T {}
    impl<T: TyConstructor2, A, B> ForAllConstructedTy2<A, B> for T {}
    impl<T: TyConstructor3, A, B, C> ForAllConstructedTy3<A, B, C> for T {}

    // blanket `witness*` implementations
    #[inline(always)]
    const fn witness1<T: TyConstructor1, A>(
        ta: <T as ForAllConstructedTy1<A>>::ConstructedType,
    ) -> T::TC1<A> {
        ta
    }
    #[inline(always)]
    const fn witness2<T: TyConstructor2, A, B>(
        ta: <T as ForAllConstructedTy2<A, B>>::ConstructedType,
    ) -> T::TC2<A, B> {
        ta
    }
    #[inline(always)]
    const fn witness3<T: TyConstructor3, A, B, C>(
        ta: <T as ForAllConstructedTy3<A, B, C>>::ConstructedType,
    ) -> T::TC3<A, B, C> {
        ta
    }

    // blanket `ConstructableTyExt*` implementations
    impl<T: ConstructableTy1 + ?Sized> const ConstructableTyExt1 for T {
        #[inline(always)]
        fn reify1(self) -> <Self::Constructor as TyConstructor1>::TC1<Self::GenericParameter1>
        where
            Self: Sized,
        {
            witness1::<Self::Constructor, Self::GenericParameter1>(self)
        }
    }
    impl<T: ConstructableTy2 + ?Sized> const ConstructableTyExt2 for T {
        #[inline(always)]
        fn reify2(
            self,
        ) -> <Self::Constructor as TyConstructor2>::TC2<
            Self::GenericParameter1,
            Self::GenericParameter2,
        >
        where
            Self: Sized,
        {
            witness2::<Self::Constructor, Self::GenericParameter1, Self::GenericParameter2>(self)
        }
    }
    impl<T: ConstructableTy3 + ?Sized> const ConstructableTyExt3 for T {
        #[inline(always)]
        fn reify3(
            self,
        ) -> <Self::Constructor as TyConstructor3>::TC3<
            Self::GenericParameter1,
            Self::GenericParameter2,
            Self::GenericParameter3,
        >
        where
            Self: Sized,
        {
            witness3::<
                Self::Constructor,
                Self::GenericParameter1,
                Self::GenericParameter2,
                Self::GenericParameter3,
            >(self)
        }
    }
}
