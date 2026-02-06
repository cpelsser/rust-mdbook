# `loop` expressions

Finally, there is a `loop` keyword which creates an endless loop. Here you must
either `break` or `return` to stop the loop:

```rust,editable
fn main() {
    let mut x = 10;
    loop {
        x = if x % 2 == 0 {
            x / 2
        } else {
            3 * x + 1
        };
        if x == 1 {
            break;
        }
    }
    println!("Final x: {x}");
}
```

<details>

`loop` is an infinite loop. Use `break` to exit, optionally with a return value.

---

**Key points for speakers:**
- `loop` is Rust's infinite loop — clearer intent than `while true`.
- The example shows the Collatz conjecture (3n+1 problem) — a fun mathematical puzzle.
- `loop` can return a value via `break value;` — this is unique to Rust.
- The compiler knows `loop` without `break` never terminates, enabling special optimizations.

**Common student questions:**
- *"Why not just use `while true`?"* - `loop` has clearer intent and the compiler treats it specially. With `while true`, the compiler can't always prove the loop runs at least once.
- *"Can loop return a value?"* - Yes! Use `break value;` to return from a loop. All `break` statements must return the same type.
- *"What's the Collatz conjecture?"* - Starting from any positive integer, if even divide by 2, if odd multiply by 3 and add 1. The conjecture says you always reach 1. Unproven!

**Demo extension:**
```rust
let result = loop {
    x = x / 2;
    if x < 10 {
        break x * 2;  // Returns a value!
    }
};
```

</details>
