use crate::typelevel::is::Is;

// Three generic structs.
struct One<T: ?Sized>(Box<T>);
struct Two<T: ?Sized>(Box<T>);
struct Three<T: ?Sized>(Box<T>);

// All three implement one trait.
trait Trait<T: ?Sized> {
    fn new(v: T) -> Self
    where
        Self: Sized,
        T: Sized;
    fn get(self) -> T
    where
        T: Sized;
}
impl<T: ?Sized> Trait<T> for One<T> {
    fn new(v: T) -> Self
    where
        Self: Sized,
        T: Sized,
    {
        Self(Box::new(v))
    }
    fn get(self) -> T
    where
        T: Sized,
    {
        *self.0
    }
}
impl<T: ?Sized> Trait<T> for Two<T> {
    fn new(v: T) -> Self
    where
        Self: Sized,
        T: Sized,
    {
        Self(Box::new(v))
    }
    fn get(self) -> T
    where
        T: Sized,
    {
        *self.0
    }
}
impl<T: ?Sized> Trait<T> for Three<T> {
    fn new(v: T) -> Self
    where
        Self: Sized,
        T: Sized,
    {
        Self(Box::new(v))
    }
    fn get(self) -> T
    where
        T: Sized,
    {
        *self.0
    }
}

// *Two* of them implement a second trait.
trait Meow {
    fn meow(&self) {
        println!("meow!");
    }
}
impl<T: ?Sized> Meow for Two<T> {}
impl<T: ?Sized> Meow for Three<T> {}

// Using the "family" pattern to allow parametricity over the three structs.
trait Family {
    type Assoc<T: ?Sized>: Trait<T> + ?Sized;
}
struct OneFamily;
struct TwoFamily;
struct ThreeFamily;
impl Family for OneFamily {
    type Assoc<T: ?Sized> = One<T>;
}
impl Family for TwoFamily {
    type Assoc<T: ?Sized> = Two<T>;
}
impl Family for ThreeFamily {
    type Assoc<T: ?Sized> = Three<T>;
}

// A wrapper struct that uses the family.
struct Wrapper<F: Family + ?Sized>
where
    F::Assoc<u32>: Sized,
{
    int: F::Assoc<u32>,
    float: F::Assoc<f64>,
}
impl<F: Family + ?Sized> Wrapper<F>
where
    F::Assoc<u32>: Sized,
{
    fn new() -> Self
    where
        F::Assoc<f64>: Sized,
    {
        Self {
            int: F::Assoc::new(42),
            float: F::Assoc::new(4.2),
        }
    }
}

// A helper-trait which "restricts" Family's GAT output

trait MeowFamily: Family {
    type MeowAssoc<T: ?Sized>: Is<Ty = Self::Assoc<T>> + Trait<T> + Meow + ?Sized;
}

impl<F: Family + ?Sized> MeowFamily for F
where
for<T> Self::Assoc<T>: Meow,
{
type MeowAssoc<T: ?Sized> = Self::Assoc<T>;
}

// Use type-witness methods to reify to the right type
impl<M: MeowFamily + ?Sized> Wrapper<M>
where
    M::Assoc<u32>: Sized,
    M::Assoc<f64>: Sized,
{
    fn speak(&self) {
        M::MeowAssoc::inv_refl_ref(&self.int).meow();
        M::MeowAssoc::inv_refl_ref(&self.float).meow();
    }
}

fn main() {
    let one = Wrapper::<OneFamily>::new();
    let two = Wrapper::<TwoFamily>::new();
    let three = Wrapper::<ThreeFamily>::new();

    // one.speak(); // doesn't (and shouldn't) work here, but these do:
    two.speak();
    three.speak();
}

