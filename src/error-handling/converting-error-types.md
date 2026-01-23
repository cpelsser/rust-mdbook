# Converting Error Types

The effective expansion of `?` is a little more complicated than previously indicated:

```rust,ignore
expression?
```

works the same as

```rust,ignore
match expression {
    Ok(value) => value,
    Err(err)  => return Err(From::from(err)),
}
```

The `From::from` call here means we attempt to convert the error type to the
type returned by the function:

<details>

**Key points for speakers:**
- The `?` operator does automatic error conversion via `From::from`.
- This is why `?` works across different error types — if `From` is implemented.
- You can implement `From<SourceError> for TargetError` to enable `?` conversion.
- This pattern enables composable error handling across library boundaries.

**Common student questions:**
- *"Why From::from and not just From?"* - `From::from` is the method. The trait is `From`, the method is `from`. Type inference figures out which `From` implementation to use.
- *"What if there's no From impl?"* - Compile error! You need to either implement `From` or manually convert with `.map_err()`.
- *"Can I chain multiple ? with different error types?"* - Yes, as long as each error type can convert to the function's return error type.
- *"What about Box<dyn Error>?"* - Any error type that implements `Error` can convert to `Box<dyn Error>`, making it useful for prototyping.

**Demo suggestion:**
Show what happens when you use `?` without a matching `From` implementation — the error message tells you exactly what's needed.

</details>
