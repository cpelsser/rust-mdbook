# Shared State

Rust uses the type system to enforce synchronization of shared data. This is
primarily done via two types:

* [`Arc<T>`][1], atomic reference counted `T`: handles sharing between threads and
  takes care to deallocate `T` when the last reference is dropped,
* [`Mutex<T>`][2]: ensures mutually exclusive access to the `T` value.

[1]: https://doc.rust-lang.org/std/sync/struct.Arc.html
[2]: https://doc.rust-lang.org/std/sync/struct.Mutex.html

<details>

- `Arc` is like `Rc` but thread-safe (uses atomic operations).
- `Mutex` provides interior mutability that's safe across threads.
- Combine them as `Arc<Mutex<T>>` to share mutable state between threads.
- `RwLock<T>` is an alternative that allows multiple readers or one writer.
- Rust's type system prevents data races at compile time.

</details>
