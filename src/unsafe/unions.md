# Unions

Unions are like enums, but you need to track the active field yourself:

```rust,editable
#[repr(C)]
union MyUnion {
    i: u8,
    b: bool,
}

fn main() {
    let u = MyUnion { i: 42 };
    println!("int: {}", unsafe { u.i });
    println!("bool: {}", unsafe { u.b });  // Undefined behavior!
}
```

<details>

**Key points for speakers:**
- Unions share memory between all fields — only one is valid at a time.
- Reading a union field is always unsafe — Rust can't track which field is active.
- `#[repr(C)]` ensures C-compatible memory layout.
- Mostly used for C FFI; Rust enums are usually preferable.

**Common student questions:**
- *"Why is reading unsafe?"* - Reading the wrong field reinterprets bits as the wrong type, potentially causing undefined behavior.
- *"How is this different from an enum?"* - Enums have a discriminant (tag) that tracks the active variant. Unions don't — you must track it yourself.
- *"When would I use a union?"* - C FFI (C unions), memory-mapped hardware registers, or rare performance optimizations.
- *"What about the bool read?"* - It's UB because `42` isn't a valid bool representation (only `0` and `1` are).

**Safer alternatives:**
- For FFI: Consider generating bindings with `bindgen`
- For type punning: Use `zerocopy` crate or `std::mem::transmute`
- For tagged unions: Use Rust enums

**Warning:**
The example shows UB intentionally. In production, never read a union field unless you're certain it's the active one.

</details>
