# `Send`

> A type `T` is [`Send`][1] if it is safe to move a `T` value to another thread.

The effect of moving ownership to another thread is that _destructors_ will run
in that thread. So the question is when you can allocate a value in one thread
and deallocate it in another.

[1]: https://doc.rust-lang.org/std/marker/trait.Send.html

<details>

`Send` means a type can safely move to another thread. Most types are `Send` automatically.

---

**Key points for speakers:**
- `Send` is a marker trait with no methods — it's purely a compile-time guarantee.
- A type is `Send` if ownership can safely transfer to another thread.
- Most types are automatically `Send` if all their fields are `Send`.
- The key insight: if destructors run on a different thread, is that safe?

**Common student questions:**
- *"Why does it matter where destructors run?"* - Some resources (like OS handles, thread-local storage) must be released on the same thread that created them.
- *"Do I need to implement Send manually?"* - Almost never. The compiler derives it automatically. You only implement it manually for unsafe abstractions.
- *"What happens if I try to send a non-Send type?"* - The compiler will reject it with a clear error message.

**Demo suggestion:**
```rust
use std::rc::Rc;
use std::thread;

fn main() {
    let rc = Rc::new(5);
    // This won't compile:
    // thread::spawn(move || println!("{}", rc));
}
```

</details>
