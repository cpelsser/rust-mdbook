# Closure Syntax

Closures are created with vertical bars: `|..| ..`.

```rust,editable
fn main() {
    // Argument and return type can be inferred for lightweight syntax:
    let double_it = |n| n * 2;
    dbg!(double_it(50));

    // Or we can specify types and bracket the body to be fully explicit:
    let add_1f32 = |x: f32| -> f32 { x + 1.0 };
    dbg!(add_1f32(50.));
}
```

<details>

**Key points for speakers:**
- `|args| body` is the basic closure syntax — pipes instead of parentheses.
- Type inference is powerful — often no annotations needed.
- Single-expression bodies don't need braces; multi-statement bodies do.
- Without captures, closures are similar to nested functions.

**Common student questions:**
- *"How is this different from a function?"* - Closures can capture variables from their environment; functions cannot.
- *"Why pipes instead of parentheses?"* - Distinguishes closures from function calls syntactically.
- *"When do I need type annotations?"* - When inference fails (ambiguous) or for documentation clarity.
- *"What's dbg!?"* - A debug macro that prints the expression, its value, and returns the value.

**Comparison with other languages:**
- JavaScript: `(n) => n * 2`
- Python: `lambda n: n * 2`
- Rust: `|n| n * 2`

**Demo suggestion:**
Show that the closure type cannot be named:
```rust
let f = |x| x + 1;
// fn takes_closure(f: ???) { }  // Can't write the type!
fn takes_closure(f: impl Fn(i32) -> i32) { }  // Use trait instead
```

</details>
