# Exercise: Iterator Method Chaining

In this exercise, you will need to find and use some of the provided methods in
the [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html) trait
to implement a complex calculation.

Use an iterator expression and `collect` the result to construct the return value.

```rust,editable
/// Calculate the differences between elements of `values` offset by `offset`,
/// wrapping around from the end of `values` to the beginning.
///
/// Element `n` of the result is `values[(n+offset)%len] - values[n]`.
fn offset_differences(offset: usize, values: Vec<i32>) -> Vec<i32> {
    todo!()
}

#[test]
fn test_offset_one() {
    assert_eq!(offset_differences(1, vec![1, 3, 5, 7]), vec![2, 2, 2, -6]);
    assert_eq!(offset_differences(1, vec![1, 3, 5]), vec![2, 2, -4]);
    assert_eq!(offset_differences(1, vec![1, 3]), vec![2, -2]);
}

#[test]
fn test_larger_offsets() {
    assert_eq!(offset_differences(2, vec![1, 3, 5, 7]), vec![4, 4, -4, -4]);
    assert_eq!(offset_differences(3, vec![1, 3, 5, 7]), vec![6, -2, -2, -2]);
    assert_eq!(offset_differences(4, vec![1, 3, 5, 7]), vec![0, 0, 0, 0]);
    assert_eq!(offset_differences(5, vec![1, 3, 5, 7]), vec![2, 2, 2, -6]);
}

#[test]
fn test_degenerate_cases() {
    assert_eq!(offset_differences(1, vec![0]), vec![0]);
    assert_eq!(offset_differences(1, vec![1]), vec![0]);
    let empty: Vec<i32> = vec![];
    assert_eq!(offset_differences(1, empty), vec![]);
}

fn main() {
    let result = offset_differences(1, vec![1, 3, 5, 7]);
    println!("Result: {:?}", result);
}
```

<details>
<summary>Hints</summary>

- Look at the `iter()`, `cycle()`, `skip()`, `zip()`, and `map()` methods.
- `cycle()` creates an infinite iterator that repeats the original.
- `zip()` combines two iterators into pairs.
- Don't forget to `collect()` at the end!

</details>
