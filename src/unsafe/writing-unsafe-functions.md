# Writing Unsafe Functions

You can mark your own functions as `unsafe` if they require particular conditions to avoid undefined
behaviour.

```rust,editable
/// Swaps the values pointed to by the given pointers.
///
/// # Safety
///
/// The pointers must be valid and properly aligned.
unsafe fn swap(a: *mut u8, b: *mut u8) {
    let temp = *a;
    *a = *b;
    *b = temp;
}

fn main() {
    let mut a = 42;
    let mut b = 66;

    // Safe because ...
    unsafe {
        swap(&mut a, &mut b);
    }

    println!("a = {}, b = {}", a, b);
}
```

<details>

**Key points for speakers:**
- `# Safety` doc comment is critical — documents what callers must guarantee.
- The `// Safe because ...` comment documents why the preconditions are met.
- Unsafe functions can contain unsafe operations without nested `unsafe` blocks.
- `#[deny(unsafe_op_in_unsafe_fn)]` is recommended — makes unsafe operations explicit.

**Common student questions:**
- *"Why document safety requirements?"* - It's the contract between function writer and caller. Without it, callers can't know what to guarantee.
- *"Is my unsafe function always called unsafely?"* - Yes, callers must wrap calls in `unsafe {}` and verify preconditions.
- *"What if I forget to document safety?"* - Clippy can warn you. It's a best practice, not enforced by the compiler.
- *"When should a function be unsafe?"* - When it has preconditions that, if violated, cause undefined behavior.

**Demo suggestion:**
Add `#[deny(unsafe_op_in_unsafe_fn)]` and show how the code needs to be restructured with explicit `unsafe` blocks inside.

**Best practice:**
```rust
#[deny(unsafe_op_in_unsafe_fn)]
unsafe fn swap(a: *mut u8, b: *mut u8) {
    // SAFETY: Caller guarantees pointers are valid and aligned
    unsafe {
        let temp = *a;
        *a = *b;
        *b = temp;
    }
}
```

</details>
