# Test Modules

Unit tests are often put in a nested module (run tests on the
[Playground](https://play.rust-lang.org/)):

```rust,editable
fn helper(a: &str, b: &str) -> String {
    format!("{a} {b}")
}

pub fn main() {
    println!("{}", helper("Hello", "World"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_helper() {
        assert_eq!(helper("foo", "bar"), "foo bar");
    }
}
```

* This lets you unit test private helpers.
* The `#[cfg(test)]` attribute is only active when you run `cargo test`.

<details>

- `#[cfg(test)]` means the module is only compiled during testing.
- `use super::*;` imports everything from the parent module.
- Test modules can access private functions in the same file.
- This pattern keeps tests close to the code they test.

</details>
