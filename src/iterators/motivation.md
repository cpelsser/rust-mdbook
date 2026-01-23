# Motivating Iterators

If you want to iterate over the contents of an array, you'll need to define:

- Some state to keep track of where you are in the iteration process, e.g. an
  index.
- A condition to determine when iteration is done.
- Logic for updating the state of iteration each loop.
- Logic for fetching each element using that iteration state.

In a C-style for loop you declare these things directly:

```c,editable
for (int i = 0; i < array_len; i += 1) {
    int elem = array[i];
}
```

In Rust we bundle this state and logic together into an object known as an
"iterator".

<details>

**Key points for speakers:**
- C-style loops mix iteration mechanics with business logic.
- Iterators separate "how to traverse" from "what to do with elements."
- Rust's `for` loop works with anything implementing `Iterator`.
- This abstraction enables powerful composition and optimization.

**Common student questions:**
- *"Why not just use C-style for loops?"* - Error-prone (off-by-one errors), less composable, harder to optimize.
- *"Are iterators slower?"* - No! Zero-cost abstractions — compiles to the same machine code as hand-written loops.
- *"What's the benefit?"* - Cleaner code, fewer bugs, and iterator adapters (`map`, `filter`, etc.) compose elegantly.
- *"Can I still access the index?"* - Yes, use `.enumerate()`.

**Rust equivalent:**
```rust
let array = [2, 4, 6, 8];
for elem in array {
    // use elem
}
```

**The abstraction advantage:**
```rust
// C-style thinking: manual index manipulation
// Iterator thinking: "for each element, do X"
array.iter()
     .filter(|x| **x > 3)
     .map(|x| x * 2)
     .sum::<i32>()
```

</details>
