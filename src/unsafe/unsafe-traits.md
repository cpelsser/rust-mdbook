# Implementing Unsafe Traits

Like with functions, you can mark a trait as `unsafe` if the implementation must guarantee
particular conditions to avoid undefined behaviour.

For example, the `zerocopy` crate has an unsafe trait that looks
[something like this](https://docs.rs/zerocopy/latest/zerocopy/trait.AsBytes.html):

```rust,editable
use std::mem::size_of_val;
use std::slice;

/// ...
/// # Safety
/// The type must have a defined representation and no padding.
pub unsafe trait AsBytes {
    fn as_bytes(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self as *const Self as *const u8, size_of_val(self))
        }
    }
}

// Safe because u32 has a defined representation and no padding.
unsafe impl AsBytes for u32 {}
```

<details>

**Key points for speakers:**
- Unsafe traits have invariants that the compiler can't verify.
- `unsafe impl` means "I promise this type satisfies the trait's requirements."
- The `# Safety` doc comment documents what implementers must guarantee.
- Common unsafe traits: `Send`, `Sync`, `GlobalAlloc`.

**Common student questions:**
- *"Why is implementing a trait unsafe?"* - The trait's methods rely on invariants. If the impl violates them, the methods become unsound.
- *"What about Send and Sync?"* - They're unsafe because the compiler can't verify thread safety for all cases. Most types auto-implement them, but raw pointers opt out by default.
- *"When would I implement an unsafe trait?"* - Rarely. Usually for FFI types, custom allocators, or when wrapping unsafe primitives.
- *"What does 'defined representation' mean?"* - The memory layout is specified (e.g., `#[repr(C)]`). Rust's default representation can change between compiler versions.

**Additional notes:**
- The `# Safety` section is a documentation convention, not enforced by the compiler.
- `zerocopy` is useful for serialization without copies — great for network protocols.

</details>
