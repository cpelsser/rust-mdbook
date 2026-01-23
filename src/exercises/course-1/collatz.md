# Exercise: Collatz Sequence

The [Collatz Sequence](https://en.wikipedia.org/wiki/Collatz_conjecture) is
defined as follows, for an arbitrary n₁ greater than zero:

- If *nᵢ* is 1, then the sequence terminates at *nᵢ*.
- If *nᵢ* is even, then *nᵢ₊₁ = nᵢ / 2*.
- If *nᵢ* is odd, then *nᵢ₊₁ = 3 * nᵢ + 1*.

For example, beginning with *n₁* = 3:

- 3 is odd, so *n₂* = 3 * 3 + 1 = 10;
- 10 is even, so *n₃* = 10 / 2 = 5;
- 5 is odd, so *n₄* = 3 * 5 + 1 = 16;
- 16 is even, so *n₅* = 16 / 2 = 8;
- 8 is even, so *n₆* = 8 / 2 = 4;
- 4 is even, so *n₇* = 4 / 2 = 2;
- 2 is even, so *n₈* = 1; and
- the sequence terminates.

Write a function to calculate the length of the Collatz sequence for a given
initial `n`.

```rust,editable,should_panic
/// Determine the length of the collatz sequence beginning at `n`.
fn collatz_length(mut n: i32) -> u32 {
    todo!("Implement this")
}

fn main() {
    println!("Length: {}", collatz_length(11)); // should be 15
}
```

<details>
<summary>Hints</summary>

- Use a `while` loop to iterate until `n` reaches 1.
- Use `if` expressions to check if `n` is even or odd.
- Remember to count each step!

</details>
