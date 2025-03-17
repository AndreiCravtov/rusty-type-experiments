use crate::family_pattern::typeclasses::semigroup::Semigroup;

pub trait Monoid: Semigroup {
    fn mempty() -> Self;
}
