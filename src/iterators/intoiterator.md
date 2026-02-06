# `IntoIterator`

The `Iterator` trait tells you how to _iterate_ once you have created an
iterator. The related trait
[`IntoIterator`](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html)
defines how to create an iterator for a type. It is used automatically by the
`for` loop.

```rust,editable
struct Grid {
    x_coords: Vec<u32>,
    y_coords: Vec<u32>,
}

impl IntoIterator for Grid {
    type Item = (u32, u32);
    type IntoIter = GridIter;
    fn into_iter(self) -> GridIter {
        GridIter { grid: self, i: 0, j: 0 }
    }
}

struct GridIter {
    grid: Grid,
    i: usize,
    j: usize,
}

impl Iterator for GridIter {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<(u32, u32)> {
        if self.i >= self.grid.x_coords.len() {
            self.i = 0;
            self.j += 1;
            if self.j >= self.grid.y_coords.len() {
                return None;
            }
        }
        let res = Some((self.grid.x_coords[self.i], self.grid.y_coords[self.j]));
        self.i += 1;
        res
    }
}

fn main() {
    let grid = Grid { x_coords: vec![3, 5, 7, 9], y_coords: vec![10, 20, 30, 40] };
    for (x, y) in grid {
        println!("point = {x}, {y}");
    }
}
```

<details>

`IntoIterator` creates an iterator from a value. `for` loops call `into_iter()` automatically.

---

**Key points for speakers:**
- `IntoIterator` is what `for` loops call — it creates an iterator from a value.
- `into_iter(self)` consumes the collection by default.
- Collections also implement `IntoIterator` for `&T` and `&mut T`.
- Two associated types: `Item` (element type) and `IntoIter` (iterator type).

**Common student questions:**
- *"Why can't I iterate twice?"* - `into_iter(self)` consumes ownership. Use `&grid` or implement `IntoIterator` for `&Grid`.
- *"What's the difference between iter() and into_iter()?"* - `iter()` borrows elements, `into_iter()` may consume or borrow depending on the receiver.
- *"How does for know which to call?"* - `for x in collection` calls `collection.into_iter()`. `for x in &collection` calls `(&collection).into_iter()`.
- *"Do I need to implement both traits?"* - Usually yes: `IntoIterator` on your type, and `Iterator` on a separate iterator struct.

**Three ways to iterate:**
```rust
let v = vec![1, 2, 3];
for x in v { }      // into_iter(), consumes v
for x in &v { }     // iter(), borrows v
for x in &mut v { } // iter_mut(), mutably borrows v
```

**Exercise:**
Try iterating over `grid` twice. Then implement `IntoIterator for &Grid` to fix it.

</details>
