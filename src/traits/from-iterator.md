# FromIterator

`FromIterator` lets you build a collection from an `Iterator`.

```rust,editable
fn main() {
    let primes = vec![2, 3, 5, 7];
    let prime_squares = primes
        .into_iter()
        .map(|prime| prime * prime)
        .collect::<Vec<_>>();
}
```

<details>

`Iterator` implements
`fn collect<B>(self) -> B
where
    B: FromIterator<Self::Item>,
    Self: Sized`
`collect` enables to convert an interator into a collection. Here it enables to reconstruct a vector.

There are also implementations which let you do cool things like convert an
`Iterator<Item = Result<V, E>>` into a `Result<Vec<V>, E>`.

Add this line to print the computed squares:
`println!("{:?}", prime_squares)`

</details>
