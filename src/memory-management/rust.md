# Memory Management in Rust

Memory management in Rust is a mix:

* Safe and correct like Java, but without a garbage collector.
* Depending on which abstraction (or combination of abstractions) you choose, can be a single unique pointer, reference counted, or atomically reference counted.
* Scope-based with automatic cleanup (like Python's `with`, but for everything).
* A Rust user can choose the right abstraction for the situation, some even have no cost at runtime like C.

It achieves this by modeling _ownership_ explicitly.

<details>

Rust enforces memory safety at compile time through ownership — no GC, no manual free.

---

**Key points for speakers:**
- Rust's key innovation: compile-time ownership tracking with zero runtime cost.
- The ownership model gives you C's performance with Java's safety guarantees.
- Different smart pointers for different needs: `Box` (single owner), `Rc` (shared, single-threaded), `Arc` (shared, multi-threaded).
- The `Drop` trait is Rust's destructor — called automatically when a value goes out of scope.

**Common student questions:**
- *"How can it be safe without a garbage collector?"* - The compiler tracks ownership and ensures memory is freed exactly once, at the right time.
- *"What if I need shared ownership?"* - Use `Rc<T>` (single-threaded) or `Arc<T>` (multi-threaded) for reference counting.
- *"What's the Drop trait?"* - It's like a destructor. Implement `fn drop(&mut self)` to run cleanup code when a value is dropped.
- *"Can Rust reject valid programs?"* - Yes, the borrow checker is conservative. Sometimes you need `unsafe` or refactoring to satisfy it.

**Teaching tip:**
Emphasize that this is the "aha moment" — Rust gets the best of all worlds by using the type system to enforce memory safety at compile time.

</details>

[Box]: https://doc.rust-lang.org/std/boxed/struct.Box.html
[Vec]: https://doc.rust-lang.org/std/vec/struct.Vec.html
[Rc]: https://doc.rust-lang.org/std/rc/struct.Rc.html
[Arc]: https://doc.rust-lang.org/std/sync/struct.Arc.html
[Drop]: https://doc.rust-lang.org/std/ops/trait.Drop.html
