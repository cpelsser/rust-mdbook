# Calling Unsafe Functions

A function or method can be marked `unsafe` if it has extra preconditions you
must uphold to avoid undefined behaviour:

```rust,editable
fn main() {
    let emojis = "🗻∈🌏";

    // Safe because the indices are in the correct order, within the bounds of
    // the string slice, and lie on UTF-8 sequence boundaries.
    unsafe {
        println!("{}", emojis.get_unchecked(0..4));
        println!("{}", emojis.get_unchecked(4..7));
        println!("{}", emojis.get_unchecked(7..11));
    }
}
```

<details>

- Unsafe functions have preconditions that the compiler cannot verify.
- You must wrap calls to unsafe functions in an `unsafe {}` block.
- This signals to readers that extra care is needed to verify correctness.
- The `_unchecked` suffix is a convention for functions that skip safety checks.
- Violating the preconditions causes undefined behavior (UB).

</details>
