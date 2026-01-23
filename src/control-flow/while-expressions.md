# `while` expressions

The `while` keyword works very similar to other languages:

```rust,editable
fn main() {
    let mut x = 10;
    while x != 1 {
        x = if x % 2 == 0 {
            x / 2
        } else {
            3 * x + 1
        };
    }
    println!("Final x: {x}");
}
```

<details>

**Key points for speakers:**
- `while` works like in most languages — nothing surprising here.
- The condition must be a `bool` — no truthy/falsy values like in JavaScript.
- Rust also has `while let` for pattern matching in loop conditions.
- Notice `if` is used as an expression inside the loop to assign to `x`.

**Common student questions:**
- *"Can I use integers as conditions like in C?"* - No, Rust requires explicit boolean conditions. Use `x != 0` instead of just `x`.
- *"What's `while let`?"* - It's like `if let` but loops: `while let Some(x) = iter.next() { ... }`. Very useful with iterators and Options.
- *"When should I use `while` vs `loop`?"* - Use `while` when you have a clear termination condition. Use `loop` when you need `break` with a value or when the exit logic is complex.

**Demo extension:**
```rust
// while let example
let mut stack = vec![1, 2, 3];
while let Some(top) = stack.pop() {
    println!("{top}");
}
```

</details>
```

