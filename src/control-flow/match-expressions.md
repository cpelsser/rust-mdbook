# `match` expressions

The `match` keyword is used to match a value against one or more patterns. In
that sense, it works like a series of `if let` expressions:

```rust,editable
fn main() {
    match std::env::args().next().as_deref() {
        Some("cat") => println!("Will do cat things"),
        Some("ls")  => println!("Will ls some files"),
        Some("mv")  => println!("Let's move some files"),
        Some("rm")  => println!("Uh, dangerous!"),
        None        => println!("Hmm, no program name?"),
        _           => println!("Unknown program name!"),
    }
}
```

Like `if let`, each match arm must have the same type. The type is the last
expression of the block, if any. In the example above, the type is `()`.

See [pattern matching](../pattern-matching.md) for more details on patterns in
Rust.

<details>

**Key points for speakers:**
- `match` must be exhaustive — all possible values must be covered.
- The `_` pattern is a catch-all that matches anything (like `default` in switch).
- `match` is an expression — all arms must return the same type.
- Pattern matching is one of Rust's most powerful features.

**Common student questions:**
- *"How is this different from switch in C/Java?"* - No fallthrough (each arm is independent), must be exhaustive, can destructure values, and it's an expression that returns a value.
- *"What does `as_deref()` do?"* - It converts `Option<String>` to `Option<&str>`, allowing us to match against string literals.
- *"Why underscore instead of `else`?"* - `_` is a pattern that matches any value. It's more consistent with Rust's pattern matching system.
- *"What if I forget a case?"* - The compiler will error! This is a huge advantage over switch statements.

**Demo suggestion:**
Show what happens when you remove the `_` arm — the compiler lists all unhandled cases (infinite in this case since it's `&str`).

</details>
