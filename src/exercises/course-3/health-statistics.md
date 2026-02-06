# Health Statistics

{{#include ../../../third_party/rust-on-exercism/health-statistics.md}}

Copy the code below to <https://play.rust-lang.org/> and fill in the missing
methods:

```rust,should_panic
// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

{{#include ../../../third_party/rust-on-exercism/health-statistics.rs}}

fn main() {
    let bob = User::new(String::from("Bob"), 32, 155.2);
    println!("I'm {} and my age is {}", bob.name(), bob.age());
}

#[test]
fn test_weight() {
    let bob = User::new(String::from("Bob"), 32, 155.2);
    assert_eq!(bob.weight(), 155.2);
}

#[test]
fn test_set_age() {
    let mut bob = User::new(String::from("Bob"), 32, 155.2);
    assert_eq!(bob.age(), 32);
    bob.set_age(33);
    assert_eq!(bob.age(), 33);
}
```

<details>

**Exercise guidance for speakers:**
- This exercise practices basic struct definitions and methods.
- Students implement getters, setters, and a constructor.
- Focus on `&self` vs `&mut self` for method receivers.

**Key concepts practiced:**
1. Struct definition with named fields.
2. Constructor pattern (`fn new(...) -> Self`).
3. Getter methods (`&self`) that borrow.
4. Setter methods (`&mut self`) that mutate.
5. Basic lifetime of references returned from getters.

**Hints to give if stuck:**
- `name()` should return `&str`, not `&String` — use `&self.name` or `self.name.as_str()`.
- `set_age` takes `&mut self` to modify the struct.
- The constructor creates a new `User` with the given values.

**Discussion points:**
- Why return `&str` from `name()` instead of `String`?
- What happens if we try to call `set_age` on an immutable `User`?

</details>
