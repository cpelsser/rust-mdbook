# Fearless Concurrency

Rust has full support for concurrency using OS threads with mutexes and
channels.

The Rust type system plays an important role in making many concurrency bugs
compile time bugs. This is often referred to as _fearless concurrency_ since you
can rely on the compiler to ensure correctness at runtime.

<details>

Rust prevents data races at compile time. The type system ensures thread safety.

---

**Key points for speakers:**
- "Fearless concurrency" means the compiler catches data races at compile time.
- Rust doesn't prevent all concurrency bugs (deadlocks are still possible), but it prevents data races.
- The ownership system naturally extends to threads: data is either moved to one thread or shared safely.

**Common student questions:**
- *"What's the difference between concurrency and parallelism?"* - Concurrency is about structure (dealing with multiple things), parallelism is about execution (doing multiple things simultaneously).
- *"How is this different from Go/Java?"* - Those languages detect data races at runtime (or don't detect them at all). Rust catches them at compile time.
- *"What about async/await?"* - Rust has async too, but this course focuses on OS threads. Async is for I/O-bound tasks with many concurrent operations.

**Common bugs Rust prevents:**
- Data races (two threads accessing same data, at least one writing)
- Use-after-free across threads
- Sending non-thread-safe data to another thread

</details>
