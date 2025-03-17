use crate::family_pattern::typeclasses::applicative::{
    Applicative, ApplicativeMut, ApplicativeOnce,
};
use crate::family_pattern::typeclasses::bind::{Bind, BindMut, BindOnce};
use crate::family_pattern::ConstructableTySyntax1;
use std::convert;
use std::marker::PhantomData;
use std::ops::ControlFlow;

pub trait Monad: Applicative + Bind {}
pub trait MonadMut: ApplicativeMut + BindMut {}
pub trait MonadOnce: ApplicativeOnce + BindOnce {}

trait MonadTry: Monad {
    fn into_control_flow<A>(ma: Self::TC1<A>) -> ControlFlow<Self::TC1<convert::Infallible>, A>;
}

trait MonadTrySyntax<M: MonadTry, A>: ConstructableTySyntax1<M, A> {
    fn mtry(self) -> MonadControlFlow<M, A>;
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
struct MonadControlFlow<M: MonadTry, A>(PhantomData<M>, PhantomData<A>, M::TC1<A>);

mod impls {
    use crate::family_pattern::{
        syntax::*,
        typeclasses::applicative::{Applicative, ApplicativeMut, ApplicativeOnce},
        typeclasses::bind::{Bind, BindMut, BindOnce},
        typeclasses::monad::{
            Monad, MonadControlFlow, MonadMut, MonadOnce, MonadTry, MonadTrySyntax,
        },
        ConstructableTyExt1 as _,
    };
    use misc::ext::UniversalExt as _;
    use std::{convert, marker::PhantomData, ops};

    impl<T: Applicative + Bind> Monad for T {}
    impl<T: ApplicativeMut + BindMut> MonadMut for T {}
    impl<T: ApplicativeOnce + BindOnce> MonadOnce for T {}

    // struct impls
    impl<M: MonadTry, A> MonadControlFlow<M, A> {
        #[must_use]
        #[inline]
        pub const fn new<MA>(ma: MA) -> Self
        where
            MA: ConstructableTySyntax1<M, A>,
        {
            Self(PhantomData, PhantomData, ma.reify1())
        }
    }

    impl<M: MonadTry, A> ops::Try for MonadControlFlow<M, A> {
        type Output = A;
        type Residual = M::TC1<convert::Infallible>;

        #[inline]
        fn from_output(output: Self::Output) -> Self {
            Self::new(M::pure(output))
        }

        #[inline]
        fn branch(self) -> ops::ControlFlow<Self::Residual, Self::Output> {
            M::into_control_flow(self.2)
        }
    }

    impl<M: MonadTry, A> ops::FromResidual<M::TC1<convert::Infallible>> for MonadControlFlow<M, A> {
        /// TODO: i don't _actually_ know if this is even safe, im just pulling stuff out of my ass :)
        #[inline]
        fn from_residual(residual: M::TC1<convert::Infallible>) -> Self {
            // SAFETY: Absolutely nothing about the underlying memory is being changed or mutated.
            //         The fact that the wrapped monadic-value is of type [`convert::Infallible`]
            //         means its guaranteed to not _actually_ be present, and instead some kind of
            //         error-value is there, which won't interact with further monadic bind-chaining.
            unsafe { residual.dangerous_unchecked_cast::<Self>() }
        }
    }

    impl<M: MonadTry, A, MA: ConstructableTySyntax1<M, A>> MonadTrySyntax<M, A> for MA {
        #[inline]
        fn mtry(self) -> MonadControlFlow<M, A> {
            MonadControlFlow::new(self)
        }
    }
}
