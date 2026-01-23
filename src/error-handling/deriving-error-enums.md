# Deriving Error Enums

The [thiserror](https://docs.rs/thiserror/) crate is a popular way to create an
error enum like we did on the previous page:

```rust,editable,compile_fail
use std::{fs, io};
use std::io::Read;
use thiserror::Error;

#[derive(Debug, Error)]
enum ReadUsernameError {
    #[error("Could not read: {0}")]
    IoError(#[from] io::Error),
    #[error("Found no username in {0}")]
    EmptyUsername(String),
}

fn read_username(path: &str) -> Result<String, ReadUsernameError> {
    let mut username = String::with_capacity(100);
    fs::File::open(path)?.read_to_string(&mut username)?;
    if username.is_empty() {
        return Err(ReadUsernameError::EmptyUsername(String::from(path)));
    }
    Ok(username)
}

fn main() {
    //fs::write("config.dat", "").unwrap();
    match read_username("config.dat") {
        Ok(username) => println!("Username: {username}"),
        Err(err)     => println!("Error: {err}"),
    }
}
```

<details>

**Key points for speakers:**
- `thiserror` is the go-to crate for library error types.
- `#[error("...")]` generates the `Display` implementation.
- `#[from]` generates `From` implementation for automatic `?` conversion.
- It's a proc macro — generates code at compile time, no runtime cost.

**Common student questions:**
- *"thiserror vs anyhow — when to use which?"* - `thiserror` for libraries (structured errors), `anyhow` for applications (convenient catch-all).
- *"What does #[from] do?"* - Generates `impl From<io::Error> for ReadUsernameError`, enabling automatic conversion with `?`.
- *"Can I have multiple #[from]?"* - Yes, for different source error types. Each generates a separate `From` impl.
- *"What about error chains?"* - `thiserror` supports `#[source]` attribute to wrap underlying errors while preserving the chain.

**Additional notes:**
- `thiserror` doesn't affect your public API — users don't need to depend on it.
- Works for both enums and structs.
- The `{0}` in error messages refers to tuple fields; use `{field_name}` for named fields.

</details>
