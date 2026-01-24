# The Stack vs The Heap

* Stack: Continuous area of memory for local variables.
  * Values have fixed sizes known at compile time.
  * Extremely fast: just move a stack pointer.
  * Easy to manage: follows function calls.
  * Great memory locality.

* Heap: Storage of values outside of function calls.
  * Values have dynamic sizes determined at runtime.
  * Slightly slower than the stack: some bookkeeping needed.
  * No guarantee of memory locality.

<details>

**Key points for speakers:**
- Stack allocation is "free" — just moving a pointer. Heap requires finding free space.
- Stack size is typically limited (8MB default on Linux). Heap is limited by system memory.
- Stack is LIFO (Last In, First Out) — matches function call/return naturally.
- Understanding this distinction is crucial for understanding Rust's ownership model.

**Common student questions:**
- *"How do I know if something is on the stack or heap?"* - Local variables of known size go on stack. `Box`, `Vec`, `String` allocate on heap. The type tells you!
- *"Why is stack faster?"* - No searching for free space, no fragmentation, better cache locality (data is contiguous).
- *"What happens if I overflow the stack?"* - Stack overflow error! Deep recursion or large arrays on stack can cause this.
- *"Can I control where data goes?"* - Yes! Use `Box::new(value)` to explicitly heap-allocate.

**Visual suggestion:**
Draw a diagram showing stack growing down from high addresses and heap growing up from low addresses.

</details>
