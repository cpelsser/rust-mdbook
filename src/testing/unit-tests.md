# Unit Tests

Mark unit tests with `#[test]`:

```rust,editable
fn first_word(text: &str) -> &str {
    match text.find(' ') {
        Some(idx) => &text[..idx],
        None => &text,
    }
}

#[test]
fn test_empty() {
    assert_eq!(first_word(""), "");
}

#[test]
fn test_single_word() {
    assert_eq!(first_word("Hello"), "Hello");
}

#[test]
fn test_multiple_words() {
    assert_eq!(first_word("Hello World"), "Hello");
}
```

Use `cargo test` to find and run the unit tests.

<details>

- Unit tests are typically placed in the same file as the code being tested.
- Use `#[cfg(test)]` on a `mod tests` block to only compile tests when running
  `cargo test`.
- This lets you unit test private helpers.
- Common assertions: `assert!`, `assert_eq!`, `assert_ne!`.
- Use `#[should_panic]` for tests that should panic.
- Use `#[ignore]` to skip slow tests by default (run with `cargo test -- --ignored`).

</details>
