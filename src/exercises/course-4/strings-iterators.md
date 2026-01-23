# Strings and Iterators

In this exercise, you are implementing a routing component of a web server. The
server is configured with a number of _path prefixes_ which are matched against
_request paths_. The path prefixes can contain a wildcard character which
matches a full segment. See the unit tests below.

Copy the following code to <https://play.rust-lang.org/> and make the tests
pass. Try avoiding allocating a `Vec` for your intermediate results:


```rust
// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

{{#include strings-iterators.rs:prefix_matches}}
    unimplemented!()
}

{{#include strings-iterators.rs:unit-tests}}
```

<details>

**Exercise guidance for speakers:**
- This exercise practices iterator combinators and avoiding allocations.
- Key insight: use `zip` to pair prefix segments with path segments.
- Wildcards (`*`) match any single segment.
- Challenge: solve without collecting into intermediate Vecs.

**Key concepts practiced:**
1. String splitting and iterating over segments.
2. Using `zip` to compare sequences in parallel.
3. Pattern matching with iterators.
4. Writing allocation-free solutions with lazy iterators.

**Hints to give if stuck:**
- `split('/')` gives an iterator over path segments.
- `zip` pairs elements from two iterators.
- Check: prefix segments must either match exactly OR be `*`.
- Empty segments (from leading `/`) need handling.

**Common student mistakes:**
- Not handling trailing slashes correctly.
- Forgetting that `zip` stops at the shorter iterator.
- Allocating when iterators would suffice.

**Elegant solution pattern:**
```rust
prefix.split('/').zip(path.split('/'))
    .all(|(p, s)| p == "*" || p == s)
    && /* prefix not longer than path */
```

</details>
