# Default Methods

Traits can implement behavior in terms of other trait methods. 

This means that when defining a trait, you can:

Declare a set of methods.
Provide default implementations for some methods, which can call other methods in the same trait.
This allows you to:

Define a "core" set of methods that must be implemented by types.
Provide additional methods that build on top of the core methods, reducing boilerplate for implementors.

```rust,editable
trait Equals {
    fn equal(&self, other: &Self) -> bool;
    fn not_equal(&self, other: &Self) -> bool {
        !self.equal(other)
    }
}

#[derive(Debug)]
struct Centimeter(i16);

impl Equals for Centimeter {
    fn equal(&self, other: &Centimeter) -> bool {
        self.0 == other.0
    }
}

fn main() {
    let a = Centimeter(10);
    let b = Centimeter(20);
    println!("{a:?} equals {b:?}: {}", a.equal(&b));
    println!("{a:?} not_equals {b:?}: {}", a.not_equal(&b));
}
```
<details>

Centimeters reuses the behavior of `not_equal` from the Equals trait. It overwrites `equal`, however.

</details>
