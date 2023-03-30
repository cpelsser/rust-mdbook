# Catching the Stack Unwinding

By default, a panic will cause the stack to unwind. The unwinding can be caught:

```rust
use std::panic;

let result = panic::catch_unwind(|| {
    println!("hello!");
});
assert!(result.is_ok());

let result = panic::catch_unwind(|| {
    panic!("oh no!");
});
assert!(result.is_err());
```

* This can be useful in servers which should keep running even if a single
  request crashes.
* This does not work if `panic = 'abort'` is set in your `Cargo.toml`.

<details>

The `panic!` macro can be used to generate a panic and start unwinding its stack. While unwinding, the runtime will take care of freeing all the resources owned by the thread by calling the destructor of all its objects.

`assert!` asserts that a boolean expression is `true` at runtime. This will invoke the `panic!` macro if the provided expression cannot be evaluated to true at runtime.
</details>
