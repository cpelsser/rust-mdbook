# Important Traits

We will now look at some of the most common traits of the Rust standard library:

* `Iterator` and `IntoIterator` used in `for` loops,
* `From` and `Into` used to convert values,
* `Read` and `Write` used for IO,
* `Add`, `Mul`, ... used for operator overloading, and
* `Drop` used for defining destructors.

<details>

**Key points for speakers:**
- These traits form the foundation of idiomatic Rust code.
- Understanding these traits is essential for reading and writing Rust effectively.
- Many of these traits can be derived automatically with `#[derive(...)]`.
- The following sections will cover each in detail.

**Common student questions:**
- *"Which traits should I implement for my types?"* - At minimum: `Debug` for debugging. Consider `Clone`, `Default`, `PartialEq`, `Eq`, `Hash` based on your needs.
- *"What's the difference between From and Into?"* - They're reciprocal. Implement `From`, get `Into` for free. Prefer `From` when implementing.
- *"Are these in the prelude?"* - Some are! `Drop`, `From`, `Into`, `Iterator` are in the prelude. Others need explicit `use`.

**Teaching tip:**
This is an overview slide. Don't go deep here — save details for the individual sections.

</details>
