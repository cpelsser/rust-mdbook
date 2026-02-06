# Capturing

A closure can capture variables from the environment where it was defined.

```rust,editable
fn main() {
    let max_value = 5;
    let clamp = |v| {
        if v > max_value { max_value } else { v }
    };

    dbg!(clamp(1));
    dbg!(clamp(3));
    dbg!(clamp(5));
    dbg!(clamp(7));
    dbg!(clamp(10));
}
```

<details>

Closures capture variables from their environment. Use `move` to take ownership.

---

**Key points for speakers:**
- Closures capture variables from their environment — this is what makes them "closures."
- Capture mode is inferred: shared reference → exclusive reference → move.
- `move` keyword forces capture by value (ownership transfer).
- The borrow checker enforces capture rules at compile time.

**Common student questions:**
- *"Why can main still use max_value?"* - It's captured by reference, not moved. The closure borrows it.
- *"What if I want to mutate a captured value?"* - The closure needs `mut` and captures by `&mut`. Try `let mut max_value` and adding mutation.
- *"When do I need `move`?"* - When the closure outlives the captured variables, e.g., spawning threads.
- *"Can I capture some by move and others by reference?"* - Yes, Rust 2021 introduced disjoint capture; older editions captured whole structs.

**Interactive exercises:**
1. Make `max_value` mutable and try to change it after defining `clamp`. Why doesn't this work?
2. Add `max_value += 1` inside `clamp`. What error do you get?
3. Add `move` before `|v|`. Can you still print `max_value` afterward?

**Thread example:**
```rust
let data = vec![1, 2, 3];
std::thread::spawn(move || {
    println!("{:?}", data);  // data is moved into the thread
});
// data is no longer accessible here
```

</details>
