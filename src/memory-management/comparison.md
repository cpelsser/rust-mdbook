# Comparison

Here is a rough comparison of the memory management techniques.

## Pros of Different Memory Management Techniques

* Manual like C:
  * No runtime overhead.
* Automatic like Java:
  * Fully automatic.
  * Safe and correct.
* Scope-based like C++:
  * Partially automatic.
  * No runtime overhead.
* Compiler-enforced scope-based like Rust:
  * Enforced by compiler.
  * No runtime overhead.
  * Safe and correct.

## Cons of Different Memory Management Techniques

* Manual like C:
  * Use-after-free.
  * Double-frees.
  * Memory leaks.
* Automatic like Java:
  * Garbage collection pauses.
  * Destructor delays.
* Scope-based like C++:
  * Complex, opt-in by programmer.
  * Potential for use-after-free.
* Compiler-enforced and scope-based like Rust:
  * Some upfront complexity.
  * Can reject valid programs.

<details>

**Key points for speakers:**
- This is a summary slide — use it to reinforce the trade-offs.
- Rust's "cons" are really about learning curve, not fundamental limitations.
- "Can reject valid programs" means the borrow checker is conservative — better safe than sorry.
- In practice, most "rejected valid programs" can be restructured to satisfy the compiler.

**Common student questions:**
- *"What does 'reject valid programs' mean?"* - The borrow checker can't prove all safe programs are safe. Example: two mutable references to different array indices.
- *"Is the upfront complexity worth it?"* - For systems programming, security-critical code, or long-running services — absolutely yes.
- *"Which should I choose for my project?"* - Depends on your constraints. Rust excels where safety AND performance matter. GC languages are fine for many applications.

**Discussion prompt:**
Ask students: "For a web server handling millions of requests, which approach would you choose and why?"

</details>
