# Dangling References

Rust will statically forbid dangling references:

```rust,editable,compile_fail
fn main() {
    let ref_x: &i32;
    {
        let x: i32 = 10;
        ref_x = &x;
    }
    println!("ref_x: {ref_x}");
}
```

* A reference is said to "borrow" the value it refers to.
* Rust is tracking the lifetimes of all references to ensure they live long
  enough.
* We will talk more about borrowing when we get to ownership.

<details>

**Key points for speakers:**
- This is a compile-time error, not a runtime crash. Rust prevents the bug before the program runs.
- In C, this code would compile and cause undefined behavior (use-after-free).
- The inner `x` is dropped when its scope ends (at the `}`), but `ref_x` still tries to reference it.

**Common student questions:**
- *"Why can't Rust just extend the lifetime of x?"* - Because `x` is stack-allocated and must be freed when its scope ends. Extending it would require heap allocation.
- *"How does Rust know this is wrong?"* - The borrow checker tracks lifetimes. It sees that `ref_x` has a longer lifetime than `x`.
- *"What if I need the value to live longer?"* - Use `Clone` to copy the value, or allocate on the heap with `Box`.

</details>
