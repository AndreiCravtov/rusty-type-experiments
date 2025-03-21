//! Allows for a hack to bypass [implied-bounds](https://rust-lang.github.io/rfcs/2089-implied-bounds.html)
//! for traits. Credit to [this crate](https://docs.rs/imply-hack/latest/imply_hack/) for the hack.
//!

/// Creates an implied bound when applied as a supertrait.
///
/// ```rust
/// trait MyTrait<T>: Imply<T, Is: Bound> {} // Implies T: Bound
/// ```
pub trait Imply<T>: inner::ImplyInner<T, Is = T> {}

impl<T, U> Imply<T> for U {}

mod inner {
    pub trait ImplyInner<T> {
        type Is;
    }

    impl<T, U> ImplyInner<T> for U {
        type Is = T;
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused)]
    
    use crate::traits::Imply;

    trait MyTrait<T> {
        fn do_the_thing(value: &T);
    }

    struct Foo;

    trait FooUser<T>
    where
        // equivalent to `where Foo: MyTrait<T>`
        Self: Imply<Foo, Is: MyTrait<T>>,
    {
        fn use_value(&self, value: &T);
    }

    struct MyFooUser;

    impl<T> FooUser<T> for MyFooUser
    where
        // equivalent to `where Foo: MyTrait<T>`
        // but this should be implied (so i guess implied bounds don't work on impl-blocks...)
        Self: Imply<Foo, Is: MyTrait<T>>,
    {
        fn use_value(&self, value: &T) {
            Foo::do_the_thing(value);
        }
    }

    fn run<T, U>(value: T, user: U)
    where
        U: FooUser<T>,
    {
        user.use_value(&value);
        <Foo as MyTrait<T>>::do_the_thing(&value); // specifically in functions, implied constraints work
    }
}
