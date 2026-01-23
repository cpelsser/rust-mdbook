# Niche Optimization

```rust,editable
#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

fn main() {
    let list: List<i32> = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("{list:?}");
}
```

A `Box` cannot be empty, so the pointer is always valid and non-`null`. This
allows the compiler to optimize the memory layout:

```bob
 Stack                           Heap
.- - - - - - - - - - - - -.     .- - - - - - - - - - - - - - - - - - - - - - - -.
:                         :     :                                               :
:    list                 :     :                                               :
:   +--------+-------+    :     :    +--------+--------+    +--------+------+   :
:   | 0      | 1     |    :     : .->| 0      |  2     | .->| ////// | //// |   :
:   | "1/Tag"| o-----+----+-----+-'  | "1/Tag"|  o-----+-'  | "1/Tag"| null |   :
:   +--------+-------+    :     :    +--------+--------+    +--------+------+   :
:                         :     :                                               :
:                         :     :                                               :
`- - - - - - - - - - - - -'     '- - - - - - - - - - - - - - - - - - - - - - - -'
```

<details>

**Key points for speakers:**
- "Niche optimization" uses invalid bit patterns to store enum discriminants.
- `Box<T>` is never null, so the null pointer value represents `None` in `Option<Box<T>>`.
- This means `Option<Box<T>>` is the same size as `Box<T>` — zero overhead!
- The `Nil` variant uses null pointer instead of a separate tag byte.

**Common student questions:**
- *"What's a niche?"* - A bit pattern that's guaranteed to never occur in valid data. Null for pointers, for example.
- *"Does this work with Option?"* - Yes! `Option<Box<T>>`, `Option<&T>`, `Option<NonZeroU32>` all benefit.
- *"What's the performance impact?"* - None for runtime! It's purely a memory layout optimization.
- *"Do I need to do anything special?"* - No, the compiler does this automatically.

**Example to show:**
```rust
println!("Size of Box<i32>: {}", std::mem::size_of::<Box<i32>>());
println!("Size of Option<Box<i32>>: {}", std::mem::size_of::<Option<Box<i32>>>());
// Both print 8 on 64-bit!
```

</details>
