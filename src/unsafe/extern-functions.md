# Calling External Code

Functions from other languages might violate the guarantees of Rust. Calling
them is thus unsafe:

```rust,editable
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        // Undefined behavior if abs misbehaves.
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
```

<details>

Calling foreign functions is unsafe — Rust can't verify what external code does.

---

**Key points for speakers:**
- `extern "C"` declares a function using C calling conventions.
- All calls to extern functions are unsafe — Rust can't verify the foreign code.
- The ABI (`"C"`) determines how arguments are passed and values returned.
- This is the foundation for FFI (Foreign Function Interface).

**Common student questions:**
- *"Why is calling abs unsafe?"* - Rust can't verify what the C function does. It might access invalid memory, cause data races, etc.
- *"Isn't abs from the C standard library safe?"* - Probably, but Rust can't know that. The function signature alone doesn't prove safety.
- *"What ABIs are available?"* - `"C"` (most common), `"system"` (Windows API), `"Rust"` (default), and others. Most code uses `"C"`.
- *"How do I link to a C library?"* - Use `#[link(name = "library")]` or configure in Cargo.toml with build scripts.

**Teaching tip:**
Point out that this is how Rust interoperates with existing C codebases — essential for systems programming.

**Example extension:**
```rust
#[link(name = "m")]  // Link to libm on Unix
extern "C" {
    fn sqrt(x: f64) -> f64;
}
```

</details>
