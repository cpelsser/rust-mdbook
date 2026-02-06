# Generic Data Types

You can use generics to abstract over the concrete field type:

```rust,editable
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!("{integer:?} and {float:?}");
}
```

<details>

- Try declaring a new variable `let p = Point { x: 5, y: 10.0 };`. This won't
  compile because `x` and `y` must have the same type `T`.
- To allow different types, you would need two type parameters:
  `struct Point<T, U> { x: T, y: U }`.
- Rust infers the type `T` based on the values provided. You can also be
  explicit: `Point::<i32> { x: 5, y: 10 }`.

</details>
