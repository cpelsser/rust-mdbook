# The Double-Free Problem

To understand why Rust uses move semantics, let's look at how other languages
handle this problem.

## The Problem in C

In C, if you copy a pointer, both copies point to the same memory:

```c
char* s1 = strdup("Hello");
char* s2 = s1;  // Both point to the same memory!
free(s1);
free(s2);       // CRASH: double-free!
```

After this, you have two pointers to the same memory. If you free both, you get
a **double-free bug** — undefined behavior that often causes crashes or security
vulnerabilities.

```bob
 Stack                             Heap
.- - - - - - - - - - - - - -.     .- - - - - - - - - - - -.
:                           :     :                       :
:    s1                     :     :                       :
:   +-----------+-------+   :     :   +---+---+---+---+   :
:   | ptr       |   o---+---+--+--+-->| H | e | l | l |   :
:   +-----------+-------+   :  |  :   +---+---+---+---+   :
:                           :  |  :                       :
:    s2                     :  |  :                       :
:   +-----------+-------+   :  |  :                       :
:   | ptr       |   o---+---+--'  :                       :
:   +-----------+-------+   :     :                       :
:                           :     `- - - - - - - - - - - -'
`- - - - - - - - - - - - - -'

       Both s1 and s2 point to the same memory!
```

## Python's Solution: Reference Counting

Python avoids this with automatic memory management:

```python
s1 = "Hello"
s2 = s1  # Both reference the same object
# Python tracks references and frees memory when count reaches 0
```

* Python uses reference counting (and garbage collection for cycles).
* Memory is freed automatically when no references remain.
* Safe, but has runtime overhead (counting references, GC pauses).

## Rust's Solution: Move Semantics

Rust takes a different approach — **ownership transfer**:

```rust,editable
fn main() {
    let s1 = String::from("Hello");
    let s2 = s1;  // Ownership MOVES to s2
    // println!("{s1}");  // ERROR: s1 is no longer valid!
    println!("{s2}");    // OK: s2 owns the data
}
```

* After `let s2 = s1`, only `s2` is valid.
* The compiler prevents use of `s1` — no runtime checks needed.
* Memory is freed exactly once, when `s2` goes out of scope.

<details>

**Key points for speakers:**
- C gives you control but lets you make mistakes (double-free, use-after-free).
- Python is safe but has runtime overhead (reference counting, GC).
- Rust achieves C's performance with Python's safety via compile-time ownership tracking.

**Common student questions:**
- *"Why doesn't Rust just use reference counting like Python?"* - Reference counting has overhead (incrementing/decrementing counts). Rust's compile-time approach has zero runtime cost.
- *"What if I need shared ownership?"* - Use `Rc<T>` (single-threaded) or `Arc<T>` (multi-threaded) — Rust offers reference counting when you need it.
- *"Isn't the move confusing?"* - It takes getting used to, but the compiler guides you. You'll never have a double-free bug.

**Comparison summary:**
| Language | Approach | Safety | Performance |
|----------|----------|--------|-------------|
| C | Manual | Unsafe | Fast |
| Python | Reference counting + GC | Safe | Runtime overhead |
| Rust | Compile-time ownership | Safe | Fast |

</details>
