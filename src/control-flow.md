# Control Flow

As we have seen, `if` is an expression in Rust. It is used to conditionally
evaluate one of two blocks, but the blocks can have a value which then becomes
the value of the `if` expression. Other control flow expressions work similarly
in Rust.

<details>

In Rust, `if`, `match`, and loops are expressions — they return values.

---

**Key points for speakers:**
- In Rust, most control flow constructs are *expressions*, not just statements.
- This means `if`, `match`, `loop` can all return values.
- This enables a more functional programming style without sacrificing readability.

**Common student questions:**
- *"Why are these expressions instead of statements?"* - It reduces the need for mutable variables and makes code more concise.
- *"Do I have to use the return value?"* - No, you can ignore it. If branches return `()` (unit), it works like a traditional statement.

**Example to show:**
```rust
let x = if condition { 5 } else { 10 };  // No need for ternary operator
```

</details>
