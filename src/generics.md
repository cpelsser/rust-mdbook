# Generics

Rust supports generics, which lets you abstract an algorithm (such as sorting)
over the types used in the algorithm.

<details>

- Generics allow code reuse while maintaining type safety.
- The compiler generates specialized code for each concrete type used
  (monomorphization), so there's no runtime cost.
- You can constrain generic types with trait bounds to require certain
  capabilities.

</details>
