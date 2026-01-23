# `for` expressions

The `for` expression is closely related to the `while let` expression. It will
automatically call `into_iter()` on the expression and then iterate over it:

```rust,editable
fn main() {
    let v = vec![10, 20, 30];

    for x in v {
        println!("x: {x}");
    }
    
    for i in (0..10).step_by(2) {
        println!("i: {i}");
    }
}
```

You can use `break` and `continue` here as usual.

<details>

**Key points for speakers:**
- `for x in collection` calls `into_iter()` automatically — consumes the collection.
- Use `&collection` or `&mut collection` to iterate by reference instead.
- Ranges like `0..10` are iterators, not special syntax.
- Iterator adapters like `step_by` chain together.

**Common student questions:**
- *"What's the difference between for and while let?"* - `for` is syntactic sugar for `while let Some(x) = iter.next()`. Use `for` for clarity.
- *"How do I iterate without consuming?"* - Use `for x in &v` (immutable refs) or `for x in &mut v` (mutable refs).
- *"Can I get the index?"* - Use `.enumerate()`: `for (i, x) in v.iter().enumerate()`.
- *"What's `0..10`?"* - A `Range<i32>` that implements `Iterator`. `0..=10` includes the endpoint.

**Demo variations:**
```rust
// By reference (doesn't consume)
for x in &v { println!("{x}"); }

// With index
for (i, x) in v.iter().enumerate() {
    println!("{i}: {x}");
}

// Inclusive range
for i in 0..=10 { /* includes 10 */ }
```

</details>
