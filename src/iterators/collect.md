# `collect`

The [`collect`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect)
method lets you build a collection from an `Iterator`.

```rust,editable
fn main() {
    let primes = vec![2, 3, 5, 7];
    let prime_squares = primes.into_iter().map(|p| p * p).collect::<Vec<_>>();
    println!("prime_squares: {prime_squares:?}");
}
```

<details>

**Key points for speakers:**
- `collect()` is generic — works with any type implementing `FromIterator`.
- The turbofish (`::<>`) specifies the collection type explicitly.
- `_` lets Rust infer element types when the collection type is specified.
- Common targets: `Vec`, `HashSet`, `HashMap`, `String`.

**Common student questions:**
- *"Why do I need the turbofish?"* - `collect()` can return many types. Rust needs to know which one.
- *"Can I collect into a String?"* - Yes! `chars.collect::<String>()` works for char iterators.
- *"What about Result?"* - `Iterator<Item=Result<T,E>>` can collect to `Result<Vec<T>,E>` — stops on first error!
- *"Which is better, turbofish or type annotation?"* - Style preference. Turbofish is inline; annotation is separate.

**Powerful patterns:**
```rust
// Collecting Results: stops on first error
let results: Result<Vec<_>, _> = strings.iter()
    .map(|s| s.parse::<i32>())
    .collect();

// Collecting into HashMap
let map: HashMap<_, _> = vec![("a", 1), ("b", 2)].into_iter().collect();

// Collecting into String
let s: String = vec!['h', 'i'].into_iter().collect();
```

</details>
