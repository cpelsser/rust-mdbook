# Destructuring Arrays

You can destructure arrays, tuples, and slices by matching on their elements:

```rust,editable
{{#include ../../third_party/rust-by-example/destructuring-arrays.rs}}
```

<details>

**Key points for speakers:**
- Array destructuring works with fixed-size arrays and slices.
- Use `..` to ignore remaining elements (like rest patterns in other languages).
- Use `_` to ignore specific elements you don't need.
- Slice patterns can match variable-length data.

**Common student questions:**
- *"What's the difference between `..` and `_`?"* - `_` ignores exactly one element, `..` ignores any number (including zero).
- *"Can I bind the rest to a variable?"* - Yes! `[first, rest @ ..]` captures remaining elements in `rest`.
- *"Does this work with Vec?"* - Only via slices. Convert with `&vec[..]` or `vec.as_slice()`.
- *"What if the array is too short?"* - For fixed patterns, it's a compile error. For slice patterns, the match fails (no panic).

**Demo extension:**
```rust
let arr = [1, 2, 3, 4, 5];
if let [first, middle @ .., last] = arr {
    println!("first: {first}, middle: {middle:?}, last: {last}");
}
```

</details>
