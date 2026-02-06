# Exercise: Fibonacci

The Fibonacci sequence begins with `[0, 1]`. For `n > 1`, the next number is the
sum of the previous two.

Write a function `fib(n)` that calculates the nth Fibonacci number. When will
this function panic?

```rust,editable,should_panic
fn fib(n: u32) -> u32 {
    if n < 2 {
        // The base case.
        return todo!("Implement this");
    } else {
        // The recursive case.
        return todo!("Implement this");
    }
}

fn main() {
    let n = 20;
    println!("fib({n}) = {}", fib(n));
}
```

<details>
<summary>Hints</summary>

- This exercise is a classic introduction to recursion.
- Think about the base cases and the recursive step.
- The question "When will this function panic?" is a hint to think about integer
  overflow. The Fibonacci sequence grows quickly!
- An iterative solution is also valid - consider the trade-offs between recursion
  and iteration (e.g., performance, stack overflow for deep recursion).

</details>
