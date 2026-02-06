# Lifetimes

A borrowed value has a _lifetime_:

* The lifetime can be elided: `add(p1: &Point, p2: &Point) -> Point`.
* Lifetimes can also be explicit: `&'a Point`, `&'document str`.
* Read `&'a Point` as "a borrowed `Point` which is valid for at least the
  lifetime `a`".
* Lifetimes are always inferred by the compiler: you cannot assign a lifetime
  yourself.
  * Lifetime annotations create constraints; the compiler verifies that there is
    a valid solution.

<details>

- Lifetimes help the compiler ensure references don't outlive the data they
  refer to.
- Most of the time, lifetimes are elided (inferred) and you don't need to write
  them.
- When the compiler can't infer lifetimes, you must annotate them explicitly.
- Lifetime names like `'a` are arbitrary; `'_` means "infer this lifetime".

</details>
