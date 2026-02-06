# Closure Traits

Closures or lambda expressions have types that cannot be named. However, they
implement special [`Fn`](https://doc.rust-lang.org/std/ops/trait.Fn.html),
[`FnMut`](https://doc.rust-lang.org/std/ops/trait.FnMut.html), and
[`FnOnce`](https://doc.rust-lang.org/std/ops/trait.FnOnce.html) traits:

```rust,editable
fn apply_and_log(
    func: impl FnOnce(&'static str) -> String,
    func_name: &'static str,
    input: &'static str,
) {
    println!("Calling {func_name}({input}): {}", func(input))
}

fn main() {
    let suffix = "-itis";
    let add_suffix = |x| format!("{x}{suffix}");
    apply_and_log(&add_suffix, "add_suffix", "senior");
    apply_and_log(&add_suffix, "add_suffix", "appendix");

    let mut v = Vec::new();
    let mut accumulate = |x| {
        v.push(x);
        v.join("/")
    };
    apply_and_log(&mut accumulate, "accumulate", "red");
    apply_and_log(&mut accumulate, "accumulate", "green");
    apply_and_log(&mut accumulate, "accumulate", "blue");

    let take_and_reverse = |prefix| {
        let mut acc = String::from(prefix);
        acc.push_str(&v.into_iter().rev().collect::<Vec<_>>().join("/"));
        acc
    };
    apply_and_log(take_and_reverse, "take_and_reverse", "reversed: ");
}
```

<details>

`Fn` borrows immutably, `FnMut` borrows mutably, `FnOnce` consumes captured values.

---

**Key points for speakers:**
- Three traits form a hierarchy: `Fn` ⊂ `FnMut` ⊂ `FnOnce`.
- Trait is determined by what the closure does with captured values.
- `Fn`: borrows immutably, can call multiple times, even concurrently.
- `FnMut`: borrows mutably, can call multiple times, not concurrently.
- `FnOnce`: consumes captures, can only call once.

**Common student questions:**
- *"How do I know which trait my closure implements?"* - The compiler infers it from usage. If it mutates → `FnMut`. If it consumes → `FnOnce`. Otherwise → `Fn`.
- *"Why the hierarchy?"* - An `Fn` can do less, so it can substitute for `FnMut` or `FnOnce`. More restrictive = more flexible usage.
- *"Which should I require in my function signature?"* - Accept `FnOnce` if possible (most flexible for callers). Only require `Fn` if you need concurrent calls.
- *"What about the & and &mut in the example?"* - `&add_suffix` passes by shared ref (Fn), `&mut accumulate` by exclusive ref (FnMut), `take_and_reverse` by value (FnOnce).

**Memory aid:**
- `Fn`: Pure function (no side effects on captured state)
- `FnMut`: Mutates state (has side effects)
- `FnOnce`: Consumes state (one-shot)

**Design tip:**
When writing APIs that take closures, prefer `FnOnce` → `FnMut` → `Fn` (most to least restrictive for the implementation, most to least flexible for callers).

</details>
