# Iterators

Iterators are a powerful Rust pattern for processing sequences of values.
They are lazy, composable, and highly optimized by the compiler.

<details>

- Iterators abstract the logic of traversing a collection.
- Lazy evaluation: no work is done until you consume the iterator.
- Iterator adapters (like `map`, `filter`) are zero-cost abstractions.
- The compiler can often optimize iterator chains as well as hand-written loops.

</details>
