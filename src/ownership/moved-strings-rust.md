# Moved Strings in Rust

```rust,editable
fn main() {
    let s1: String = String::from("Rust");
    let s2: String = s1;
}
```

* The heap data from `s1` is reused for `s2`.
* When `s1` goes out of scope, nothing happens (it has been moved from).

Before move to `s2`:

```bob
 Stack                             Heap
.- - - - - - - - - - - - - -.     .- - - - - - - - - - - - - -.
:                           :     :                           :
:    s1                     :     :                           :
:   +-----------+-------+   :     :   +----+----+----+----+   :
:   | ptr       |   o---+---+-----+-->| R  | u  | s  | t  |   :
:   | len       |     4 |   :     :   +----+----+----+----+   :
:   | capacity  |     4 |   :     :                           :
:   +-----------+-------+   :     :                           :
:                           :     `- - - - - - - - - - - - - -'
:                           :
`- - - - - - - - - - - - - -'
```

After move to `s2`:

```bob
 Stack                             Heap
.- - - - - - - - - - - - - -.     .- - - - - - - - - - - - - -.
:                           :     :                           :
:    s1 "(inaccessible)"    :     :                           :
:   +-----------+-------+   :     :   +----+----+----+----+   :
:   | ptr       |   o---+---+--+--+-->| R  | u  | s  | t  |   :
:   | len       |     4 |   :  |  :   +----+----+----+----+   :
:   | capacity  |     4 |   :  |  :                           :
:   +-----------+-------+   :  |  :                           :
:                           :  |  `- - - - - - - - - - - - - -'
:    s2                     :  |
:   +-----------+-------+   :  |
:   | ptr       |   o---+---+--'
:   | len       |     4 |   :
:   | capacity  |     4 |   :
:   +-----------+-------+   :
:                           :
`- - - - - - - - - - - - - -'
```

<details>

**Key points for speakers:**
- This diagram is crucial — spend time on it! It shows move semantics visually.
- After the move, `s1` is marked as inaccessible — the compiler enforces this.
- The heap data is NOT copied — just the stack metadata (ptr, len, capacity).
- This is "zero-cost" — moving is just copying 24 bytes (3 × 8 bytes on 64-bit).

**Common student questions:**
- *"What happens if I try to use s1 after the move?"* - Compile error! Rust prevents use-after-move at compile time.
- *"Why not just copy the data?"* - Copying could be expensive for large data. Move is always cheap (fixed size metadata).
- *"How is this different from Python?"* - In Python, `s2 = s1` creates two references to the same object. In Rust, ownership transfers and `s1` becomes invalid.
- *"When would I want to copy instead?"* - Use `.clone()` explicitly when you need independent copies.

**Teaching tip:**
Walk through what happens at each line: creation, assignment (move), and going out of scope (drop of s2, nothing for s1).

</details>
