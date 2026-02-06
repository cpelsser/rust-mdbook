# Integration Tests

If you want to test your library as a client, use an integration test.

Create a `.rs` file under `tests/`:

```rust,ignore
use my_library::init;

#[test]
fn test_init() {
    assert!(init().is_ok());
}
```

These tests only have access to the public API of your crate.

<details>

- Integration tests live in the `tests/` directory at the crate root.
- Each file in `tests/` is compiled as a separate crate.
- Use `tests/common/mod.rs` for shared test utilities.
- Integration tests are only for library crates (they can't access binary internals).
- Run specific integration tests with `cargo test --test <name>`.

</details>
