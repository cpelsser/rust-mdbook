# Mutable Static Variables

It is safe to read an immutable static variable:

```rust,editable
static HELLO_WORLD: &str = "Hello, world!";

fn main() {
    println!("HELLO_WORLD: {}", HELLO_WORLD);
}
```

However, since data races can occur, it is unsafe to read and write mutable
static variables:

```rust,editable
static mut COUNTER: u32 = 0;

fn add_to_counter(inc: u32) {
    unsafe { COUNTER += inc; }  // Potential data race!
}

fn main() {
    add_to_counter(42);

    unsafe { println!("COUNTER: {}", COUNTER); }  // Potential data race!
}
```

<details>

`static mut` is unsafe due to data races. Prefer `Mutex`, `RwLock`, or atomics instead.

---

**Key points for speakers:**
- Immutable statics are safe and common — they're truly global constants.
- Mutable statics are `unsafe` because they can cause data races.
- Every access to a `mut static` requires an `unsafe` block.
- Prefer `Mutex`, `RwLock`, or atomics for shared mutable state.

**Common student questions:**
- *"Why is reading also unsafe?"* - Another thread might be writing simultaneously. Even reading during a write can observe torn values.
- *"What should I use instead?"* - `lazy_static!` or `once_cell` for lazy initialization, `Mutex<T>` for thread-safe mutation, atomics for simple counters.
- *"When is static mut acceptable?"* - Rare: embedded/no_std where you control all access, FFI globals, single-threaded programs.
- *"What's a data race?"* - Two threads accessing the same memory, at least one writing, with no synchronization. Undefined behavior in Rust.

**Additional notes:**
- `no_std` environments (embedded, kernel code) may need `static mut` because standard synchronization primitives aren't available.
- Consider `AtomicU32` for this counter example — it's safe and efficient.

**Better alternative:**
```rust
use std::sync::atomic::{AtomicU32, Ordering};
static COUNTER: AtomicU32 = AtomicU32::new(0);
fn add_to_counter(inc: u32) {
    COUNTER.fetch_add(inc, Ordering::Relaxed);
}
```

</details>
