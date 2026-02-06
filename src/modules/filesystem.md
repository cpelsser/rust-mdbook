# Filesystem Hierarchy

The module content can be omitted:

```rust,editable,compile_fail
mod garden;
```

The `garden` module content is found at:

* `src/garden.rs` (modern Rust 2018 style)
* `src/garden/mod.rs` (older Rust 2015 style)

Similarly, a `garden::vegetables` module can be found at:

* `src/garden/vegetables.rs` (modern Rust 2018 style)
* `src/garden/vegetables/mod.rs` (older Rust 2015 style)

The `crate` root is in:

* `src/lib.rs` (for a library crate)
* `src/main.rs` (for a binary crate)

<details>

`mod foo;` loads from `foo.rs` (Rust 2018) or `foo/mod.rs` (Rust 2015). Prefer the newer style.

---

**Key points for speakers:**
- Rust 2018 style (`garden.rs`) is preferred — it's cleaner and has fewer files.
- The old `mod.rs` style still works but clutters directories.
- A crate can have both `lib.rs` AND `main.rs` — the binary can use the library.
- `mod foo;` tells the compiler to look for module content in a file.

**Common student questions:**
- *"Which style should I use?"* - Use Rust 2018 style (`garden.rs`). Only use `mod.rs` if you're maintaining old code.
- *"Can I have both lib.rs and main.rs?"* - Yes! The binary (`main.rs`) can `use your_crate::something` from the library.
- *"What's the difference between a crate and a module?"* - A crate is a compilation unit (one `lib.rs` or `main.rs`). Modules organize code within a crate.
- *"How do I create nested modules?"* - Create a directory with the module name and put submodule files inside.

**Demo suggestion:**
Show `cargo new --lib mylib` and create a nested module structure to demonstrate both styles.

</details>
