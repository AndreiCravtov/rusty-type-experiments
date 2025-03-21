//! Simple, user-implementable "function" type.

/// This is a simple, user-implementable alternative to `Fn*` traits, which does **not** allow for
/// closure-like behavior. So really, it is more like Rust's `fn` pure functions.
pub trait Func<Input> {
    type Output;

    /// Call the `Func`.
    ///
    /// Notice that this does not take a self argument, which in turn means `Func` cannot behave
    /// like a closure, and is instead more like Rust's `fn` pure functions.
    fn call(i: Input) -> Self::Output;
}
