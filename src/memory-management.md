# Memory Management

Traditionally, languages have fallen into two broad categories:

* Full control via manual memory management: C, Pascal, ...
* Full safety via automatic memory management at runtime: Java, Python, Go, Haskell, ...

Rust offers a new mix:

> Full control *and* safety via compile time enforcement of correct memory
> management.

It does this with an explicit ownership concept.

First, let's refresh how memory management works.

<details>

- C gives full control but requires manual memory management, leading to bugs
  like use-after-free, double-free, and memory leaks.
- Languages with garbage collection (Java, Go, Python) are safe but have
  runtime overhead and less predictable performance.
- Rust achieves both safety and control through compile-time ownership rules.

</details>
