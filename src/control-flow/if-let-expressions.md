# `if let` expressions

If you want to match a value against a pattern, you can use `if let`:

```rust,editable
fn main() {
    let arg = std::env::args().next();
    if let Some(value) = arg {
        println!("Program name: {value}");
    } else {
        println!("Missing name?");
    }
}
```

See [pattern matching](../pattern-matching.md) for more details on patterns in
Rust.

<details>

**Key points for speakers:**
- `if let` is syntactic sugar for a `match` with one interesting arm and a wildcard.
- Use it when you only care about one pattern and want to ignore the rest.
- Can be chained with `else if let` for multiple patterns.
- Unlike `match`, `if let` is not exhaustive — you don't need to handle all cases.

**Common student questions:**
- *"When should I use if let vs match?"* - `if let` for one or two patterns, `match` for many patterns or when you need exhaustiveness checking.
- *"What about let-else?"* - `let Some(x) = opt else { return; }` is useful for early returns. Available since Rust 1.65.
- *"Can I use if let with Result?"* - Yes! `if let Ok(value) = result { ... }` is common.
- *"Why no guard clauses?"* - Design choice for simplicity. Use `match` if you need guards like `Some(x) if x > 5`.

**Demo extension:**
```rust
// let-else for early returns
let Some(value) = arg else {
    println!("Missing name?");
    return;
};
println!("Program name: {value}");
```

</details>
