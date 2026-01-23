# `if` expressions

You use `if` very similarly to how you would in other languages:

```rust,editable
fn main() {
    let mut x = 10;
    if x % 2 == 0 {
        x = x / 2;
    } else {
        x = 3 * x + 1;
    }
}
```

In addition, you can use it as an expression. This does the same as above:

```rust,editable
fn main() {
    let mut x = 10;
    x = if x % 2 == 0 {
        x / 2
    } else {
        3 * x + 1
    };
}
```

<details>

**Key points for speakers:**
- `if` is an expression, not just a statement — it returns a value.
- Both branches must return the same type (or `()` if no value needed).
- This replaces the ternary operator (`? :`) from C-like languages.
- No parentheses required around the condition (unlike C/Java).

**Common student questions:**
- *"Why no ternary operator?"* - Rust's `if` expression serves the same purpose more readably.
- *"What if branches have different types?"* - Compile error! Both must match.
- *"What happens with a trailing semicolon?"* - The branch returns `()` instead of the value. Try adding `;` after `x / 2` to see.
- *"Do I need the else?"* - Only if using `if` as an expression for a value. Without `else`, the type is `()`.

**Demo suggestion:**
Show the compile error when branches have different types:
```rust
let x = if true { 5 } else { "hello" };  // Error!
```

**Note about style:**
When used as an expression, put the result on its own line without `;` for clarity.

</details>
