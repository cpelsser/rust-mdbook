# `Iterator` Helper Methods

In addition to the `next` method that defines how an iterator behaves, the
`Iterator` trait provides 70+ helper methods that can be used to build
customized iterators.

```rust,editable
fn main() {
    let result: i32 = (1..=10) // Create a range from 1 to 10
        .filter(|x| x % 2 == 0) // Keep only even numbers
        .map(|x| x * x) // Square each number
        .sum(); // Sum up all the squared numbers

    println!("The sum of squares of even numbers from 1 to 10 is: {}", result);
}
```

<details>

Adapters (`map`, `filter`) are lazy. Consumers (`sum`, `collect`) produce a result.

---

**Key points for speakers:**
- Adapters (`map`, `filter`, `take`) return new iterators — they don't consume.
- Consumers (`sum`, `count`, `collect`) drain the iterator and produce a result.
- Adapters are lazy: chaining `.filter().map()` creates no intermediate collections.
- The compiler optimizes iterator chains into tight loops (zero-cost abstraction).

**Common student questions:**
- *"Does filter create a new Vec?"* - No! It returns a lazy iterator that skips elements on demand.
- *"What's the difference between map and for_each?"* - `map` returns an iterator; `for_each` consumes for side effects.
- *"Are closures in map/filter slow?"* - No, they're typically inlined by the compiler.
- *"What if I need the index?"* - Use `.enumerate()` to get `(index, value)` tuples.

**Common adapters:**
- `map(f)` — transform each element
- `filter(p)` — keep elements matching predicate
- `take(n)` — take first n elements
- `skip(n)` — skip first n elements
- `enumerate()` — add index to each element
- `zip(other)` — pair with another iterator

**Common consumers:**
- `collect()` — build a collection
- `sum()`, `product()` — arithmetic aggregation
- `count()` — number of elements
- `find(p)` — first matching element
- `any(p)`, `all(p)` — boolean tests

</details>
