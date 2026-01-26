# What is `no_std`?

Rust's standard library is divided into three layers:

<table>
<tr>
<th>

`core`

</th>
<th>

`alloc`

</th>
<th>

`std`

</th>
</tr>
<tr valign="top">
<td>

- Slices, `&str`
- `Option`, `Result`
- `Iterator`
- `Display`, `Debug`, `write!`
- `panic!`, `assert!`
- `Future` and `async`/`await`
- `AtomicBool`, `AtomicU32`...
- `Duration`

</td>
<td>

- `Box`, `Arc`, `Rc`
- `Vec`, `VecDeque`
- `BTreeMap`, `BinaryHeap`
- `String`, `format!`

</td>
<td>

- `HashMap`
- `Mutex`, `RwLock`, `mpsc`
- `File` and `fs`
- `println!`, `Read`, `Write`
- `Path`, `OsString`
- `net`
- `spawn`, `sleep`, `thread`
- `SystemTime`, `Instant`

</td>
</tr>
</table>

## What Each Layer Provides

- **`core`** - Always available, works everywhere (even bare metal)
- **`alloc`** - Needs a memory allocator (heap)
- **`std`** - Needs an operating system

## Declaring `no_std`

To write bare metal code, add this to your crate root:

```rust,ignore
#![no_std]
```

This tells Rust not to link the standard library. You can still use
everything from `core`.

<details>

**Key points:**

- `no_std` doesn't mean "no libraries" - you get all of `core`
- Most of what you learned still applies: `Option`, `Result`, iterators
- `HashMap` depends on random number generation (for security)
- `std` re-exports the contents of both `core` and `alloc`

**Common student questions:**

- *"Can I use `Vec` on bare metal?"* - Yes, if you provide a global allocator.
  Many embedded projects avoid dynamic allocation entirely.
- *"What about `println!`?"* - You can implement your own using UART/serial,
  or use RTT (Real-Time Transfer) for debugging.
- *"Why avoid dynamic allocation?"* - Heap fragmentation and allocation failures
  are hard to handle in safety-critical systems. Fixed-size buffers are predictable.

</details>
