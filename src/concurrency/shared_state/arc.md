# `Arc`

[`Arc<T>`][1] allows shared read-only access via its `clone` method:

```rust,editable
use std::thread;
use std::sync::Arc;

fn main() {
    let v = Arc::new(vec![10, 20, 30]);
    let mut handles = Vec::new();
    for _ in 1..5 {
        let v = v.clone();
        handles.push(thread::spawn(move || {
            let thread_id = thread::current().id();
            println!("{thread_id:?}: {v:?}");
        }));
    }

    handles.into_iter().for_each(|h| h.join().unwrap());
    println!("v: {v:?}");
}
```

[1]: https://doc.rust-lang.org/std/sync/struct.Arc.html

<details>

* `Arc` stands for "Atomic Reference Counted", a thread safe version of `Rc` that uses atomic
  operations.

  "The type Arc<T> provides shared ownership of a value of type T, allocated in the heap. Invoking clone on Arc produces a new Arc instance, which points to the same allocation on the heap as the source Arc, while increasing a reference count. When the last Arc pointer to a given allocation is destroyed, the value stored in that allocation (often referred to as “inner value”) is also dropped." from [1].

  Also see [How arc works in rust](https://medium.com/@DylanKerler1/how-arc-works-in-rust-b06192acd0a6)


* `Arc<T>` implements `Clone` whether or not `T` does. It implements `Send` and `Sync` iff `T`
  implements them both.
* `Arc::clone()` has the cost of atomic operations that get executed, but after that the use of the
  `T` is free.
* Beware of reference cycles, `Arc` does not use a garbage collector to detect them.
    * `std::sync::Weak` can help.


</details>
