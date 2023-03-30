# Monomorphization

Generic code is turned into non-generic code based on the call sites:

```rust,editable
fn main() {
    let integer = Some(5);
    let float = Some(5.0);
}
```

behaves as if you wrote

```rust,editable
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```

This is a zero-cost abstraction: you get exactly the same result as if you had
hand-coded the data structures without the abstraction.

<details>
    
[Monomorphization](https://rustc-dev-guide.rust-lang.org/backend/monomorph.html)

"In Rust, the compiler stamps out a different copy of the code of a generic function for each concrete type needed. For example, if I use a `Vec<u64>` and a `Vec<String>` in my code, then the generated binary will have two copies of the generated code for Vec: one for `Vec<u64>` and another for `Vec<String>`. The result is fast programs, but it comes at the cost of compile time (creating all those copies can take a while) and binary size (all those copies might take a lot of space)." This is different from Java, where the precise type is known at run-time and (almost) all variables are reference values (ie pointer to a heap allocated object). This has performance cost because of the dereference for each access.

</details>