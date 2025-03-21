use crate::private;

/// Emulation of `T == U` bounds with `T: Is<Ty = U>`.
/// TODO: add support for maybe HKT-mapping???
///       e.g. `refl_inner<T<_>, A>(ta: T<A>) -> T<Ty> {}` or something...??
#[const_trait]
pub trait Is: private::Sealed {
    type Ty: Is<Ty = Self> + ?Sized;

    fn refl(self) -> Self::Ty
    where
        Self: Sized;

    fn inv_refl(this: Self::Ty) -> Self
    where
        Self::Ty: Sized;

    fn refl_ref(&self) -> &Self::Ty;

    fn inv_refl_ref(this: &Self::Ty) -> &Self;

    fn refl_mut(&mut self) -> &mut Self::Ty;

    fn inv_refl_mut(this: &mut Self::Ty) -> &mut Self;
}

#[allow(clippy::inline_always)]
mod impls {
    use crate::traits::is::Is;

    impl<T: ?Sized> const Is for T {
        type Ty = T;

        #[inline(always)]
        fn refl(self) -> T
        where
            T: Sized,
        {
            self
        }

        #[inline(always)]
        fn inv_refl(this: T) -> T
        where
            T: Sized,
        {
            this
        }

        #[inline(always)]
        fn refl_ref(&self) -> &T {
            self
        }

        #[inline(always)]
        fn inv_refl_ref(this: &T) -> &T {
            this
        }

        #[inline(always)]
        fn refl_mut(&mut self) -> &mut T {
            self
        }

        #[inline(always)]
        fn inv_refl_mut(this: &mut T) -> &mut T {
            this
        }
    }
}
