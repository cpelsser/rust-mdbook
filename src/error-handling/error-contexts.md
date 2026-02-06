# Adding Context to Errors

The widely used [anyhow](https://docs.rs/anyhow/) crate can help you add
contextual information to your errors and allows you to have fewer
custom error types:

```rust,editable,compile_fail
use std::{fs, io};
use std::io::Read;
use anyhow::{Context, Result, bail};

fn read_username(path: &str) -> Result<String> {
    let mut username = String::with_capacity(100);
    fs::File::open(path)
        .context(format!("Failed to open {path}"))?
        .read_to_string(&mut username)
        .context("Failed to read")?;
    if username.is_empty() {
        bail!("Found no username in {path}");
    }
    Ok(username)
}

fn main() {
    //fs::write("config.dat", "").unwrap();
    match read_username("config.dat") {
        Ok(username) => println!("Username: {username}"),
        Err(err)     => println!("Error: {err:?}"),
    }
}
```

<details>

`anyhow` adds context to errors. Use `.context()` to enrich errors, `bail!` to return early.

---

**Key points for speakers:**
- `anyhow` is the go-to crate for application error handling.
- `.context()` wraps errors with additional information.
- `bail!` is a macro for early returns with an error message.
- `anyhow::Result<T>` = `Result<T, anyhow::Error>`.

**Common student questions:**
- *"anyhow vs thiserror?"* - `anyhow` for applications (easy, flexible), `thiserror` for libraries (structured, typed).
- *"What's the ? operator doing here?"* - Same as before, but `context()` returns a Result with enriched error info.
- *"Can I still pattern match on errors?"* - Yes, use `.downcast_ref::<MyError>()` to get the original error type.
- *"What about performance?"* - `anyhow::Error` allocates. For hot paths, consider typed errors.

**Additional notes:**
- `anyhow::Error` wraps `Box<dyn Error>` — type-erased, but flexible.
- Great for prototyping and applications where error types don't need to be part of API.
- Similar ergonomics to Go's `(T, error)` pattern.
- Use `{err:?}` (Debug) to see full error chain, `{err}` (Display) for user-facing message.

</details>
