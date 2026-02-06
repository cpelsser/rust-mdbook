# Testing

Rust and Cargo come with a simple unit test framework:

* Unit tests are supported throughout your code.

* Integration tests are supported via the `tests/` directory.

<details>

- Run tests with `cargo test`.
- Tests run in parallel by default; use `cargo test -- --test-threads=1` for
  sequential execution.
- Use `cargo test <name>` to filter tests by name.
- Tests that panic are considered failures unless marked with `#[should_panic]`.

</details>
