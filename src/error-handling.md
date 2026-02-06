# Error Handling

Error handling in Rust is done using explicit control flow:

* Functions that can have errors list this in their return type,
* There are no exceptions.

[Doc on handling errors](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)

<details>

No exceptions. Errors are explicit values in `Result<T, E>`. Use `?` to propagate.

---

**Key points for speakers:**
- Rust has NO exceptions. This is a deliberate design choice.
- Errors are values that must be explicitly handled or propagated.
- Two categories: recoverable (`Result`) and unrecoverable (`panic!`).

**Common student questions:**
- *"Why no exceptions?"* - Exceptions have hidden control flow. You can't tell from a function signature if it might throw. In Rust, errors are visible in the type signature.
- *"Isn't this more verbose?"* - The `?` operator makes it concise. And the explicitness prevents bugs from unhandled errors.
- *"When should I panic vs return Result?"* - Panic for bugs (should never happen), Result for expected failures (file not found, network error, invalid input).

**Comparison with other languages:**
- Java: checked exceptions (verbose) or unchecked (invisible)
- Python/JS: all exceptions are unchecked (easy to miss)
- Go: similar to Rust with explicit error returns, but no `?` operator
- Rust: explicit with ergonomic `?` operator

</details>