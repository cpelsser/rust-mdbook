# `Rc`

[`Rc`][1] is a reference-counted shared pointer. Use this when you need to refer
to the same data from multiple places:

```rust,editable
use std::rc::Rc;

fn main() {
    let mut a = Rc::new(10);
    let mut b = a.clone();

    println!("a: {a}");
    println!("b: {b}");
}
```

Shared references in Rust disallow mutation by default. 
If you need to mutate the data inside an `Rc`, you will need to wrap the data in
a type such as [`Cell` or `RefCell`][2], in single-threaded cases. See [`Arc`][3] if you are in a multi-threaded
context. Arc stands for "Atomically Reference counted". If you need to mutate through Arc use [`Mutex`][4], [`RwLock`][5], or one of the [`Atomic`][6] types.

[1]: https://doc.rust-lang.org/std/rc/struct.Rc.html
[2]: https://doc.rust-lang.org/std/cell/index.html
[3]: https://doc.rust-lang.org/std/sync/struct.Arc.html
[4]: https://doc.rust-lang.org/std/sync/struct.Mutex.html
[5]: https://doc.rust-lang.org/std/sync/struct.RwLock.html
[6]: https://doc.rust-lang.org/std/sync/atomic/index.html

<details>

* Similar to Python's object model — Python uses reference counting internally for all objects.
* The difference: in Rust, you explicitly choose `Rc` when you need shared ownership; it's not the default.
* `clone` is cheap: creates a pointer to the same allocation and increases the reference count.
* `make_mut` actually clones the inner value if necessary ("clone-on-write") and returns a mutable reference.

</details>
