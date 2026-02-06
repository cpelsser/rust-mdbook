# Panics

Rust will trigger a panic if a fatal error happens at runtime:

```rust,editable,should_panic
fn main() {
    let v = vec![10, 20, 30];
    println!("v[100]: {}", v[100]);
}
```

* Panics are for unrecoverable and unexpected errors.
  * Panics are symptoms of bugs in the program.
* Use non-panicking APIs (such as `Vec::get`) if crashing is not acceptable.

<details>

- By default, a panic will unwind the stack and clean up resources.
- You can use `panic = 'abort'` in `Cargo.toml` to abort immediately instead.
- `unwrap()` and `expect()` panic if called on `None` or `Err`.
- Use `Result` for recoverable errors, panics for unrecoverable ones.
- The `RUST_BACKTRACE=1` environment variable shows a stack trace on panic.

</details>
