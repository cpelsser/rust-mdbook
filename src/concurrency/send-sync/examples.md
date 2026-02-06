# Examples

## `Send + Sync`

Most types you come across are `Send + Sync`:

* `i8`, `f32`, `bool`, `char`, `&str`, ...
* `(T1, T2)`, `[T; N]`, `&[T]`, `struct { x: T }`, ...
* `String`, `Option<T>`, `Vec<T>`, `Box<T>`, ...
* `Arc<T>`: Explicitly thread-safe via atomic reference count.
* `Mutex<T>`: Explicitly thread-safe via internal locking.
* `AtomicBool`, `AtomicU8`, ...: Uses special atomic instructions.

The generic types are typically `Send + Sync` when the type parameters are
`Send + Sync`.

## `Send + !Sync`

These types can be moved to other threads, but they're not thread-safe.
Typically because of interior mutability:

* `mpsc::Sender<T>`
* `mpsc::Receiver<T>`
* `Cell<T>`
* `RefCell<T>`

## `!Send + Sync`

These types are thread-safe, but they cannot be moved to another thread:

* `MutexGuard<T>`: Uses OS level primitives which must be deallocated on the
  thread which created them.

## `!Send + !Sync`

These types are not thread-safe and cannot be moved to other threads:

* `Rc<T>`: each `Rc<T>` has a reference to an `RcBox<T>`, which contains a
  non-atomic reference count.
* `*const T`, `*mut T`: Rust assumes raw pointers may have special
  concurrency considerations.

<details>

Most types are `Send + Sync`. `Rc` is neither. `Cell`/`RefCell` are `Send` but not `Sync`.

---

**Key points for speakers:**
- This is a reference table — students don't need to memorize it, but should understand the categories.
- `Send + Sync` is the "happy path" — most types you write will be both.
- Interior mutability (`Cell`, `RefCell`) breaks `Sync` because concurrent mutation without synchronization is unsafe.
- `Rc` is the classic example of `!Send + !Sync` — its reference count isn't atomic.

**Common student questions:**
- *"Why is `MutexGuard` not `Send`?"* - Many OS mutex implementations require unlock to happen on the same thread as lock. It IS `Sync` though, because multiple threads can safely have references to it.
- *"Why is `Arc` both Send and Sync but `Rc` is neither?"* - `Arc` uses atomic operations for its reference count, making concurrent access safe. `Rc` uses non-atomic operations for performance.
- *"What about `Cell` vs `RefCell`?"* - Both are `Send` (can move to another thread) but neither is `Sync` (can't share references across threads) because they allow mutation through shared references.

**Teaching tip:**
Draw a 2x2 grid on the board with Send/!Send on one axis and Sync/!Sync on the other. Place common types in each quadrant as you discuss them.

</details>
