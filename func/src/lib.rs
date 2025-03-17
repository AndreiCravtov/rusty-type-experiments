/// Create a constant function which produces the same constant value for every argument passed to it.
#[allow(clippy::inline_always)]
#[inline(always)]
pub const fn constant<A: Clone, B>(constant: A) -> impl Fn(B) -> A {
    move |_| constant.clone()
}
