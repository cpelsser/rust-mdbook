# `Iterator` Trait

The [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html) trait
defines how an object can be used to produce a sequence of values. For example,
if we wanted to create an iterator that can produce the elements of a slice it
might look something like this:

```rust,editable
struct SliceIter<'s> {
    slice: &'s [i32],
    i: usize,
}

impl<'s> Iterator for SliceIter<'s> {
    type Item = &'s i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i == self.slice.len() {
            None
        } else {
            let next = &self.slice[self.i];
            self.i += 1;
            Some(next)
        }
    }
}

fn main() {
    let slice = &[2, 4, 6, 8];
    let iter = SliceIter { slice, i: 0 };
    for elem in iter {
        dbg!(elem);
    }
}
```

<details>

`Iterator` has one required method: `next()` returns `Some(item)` or `None` when done.

---

**Key points for speakers:**
- `Iterator` has one required method: `next()` → `Option<Item>`.
- `None` signals the end of iteration — clean, no sentinel values.
- Iterators are lazy: no work until `next()` is called.
- The 70+ helper methods are all implemented in terms of `next()`.

**Common student questions:**
- *"Why return Option instead of throwing?"* - Rust doesn't have exceptions. `None` is explicit end-of-iteration.
- *"What's `type Item`?"* - An associated type; the type of elements produced. Each iterator has exactly one item type.
- *"Can iterators be infinite?"* - Yes! `0..` produces integers forever (until overflow). Use `.take(n)` to limit.
- *"Do I need to implement all 70 methods?"* - No! Only `next()` is required. All others have default implementations.

**Demo suggestion:**
Show what happens if you call `next()` manually:
```rust
let mut iter = [1, 2, 3].into_iter();
println!("{:?}", iter.next()); // Some(1)
println!("{:?}", iter.next()); // Some(2)
println!("{:?}", iter.next()); // Some(3)
println!("{:?}", iter.next()); // None
```

**Teaching tip:**
Emphasize the lifetime in `SliceIter<'s>` — the iterator borrows the slice and can't outlive it.

</details>
