# `break` and `continue`

If you want to exit a loop early, use `break`, if you want to immediately start
the next iteration use `continue`. Both `continue` and `break` can optionally
take a label argument which is used to break out of nested loops:

```rust,editable
fn main() {
    let v = vec![10, 20, 30];
    let mut iter = v.into_iter();
    'outer: while let Some(x) = iter.next() {
        println!("x: {x}");
        let mut i = 0;
        while i < x {
            println!("x: {x}, i: {i}");
            i += 1;
            if i == 3 {
                break 'outer;
            }
        }
    }
}
```

In this case we break the outer loop after 3 iterations of the inner loop.

<details>

`break` exits a loop, `continue` skips to the next iteration. Use labels (`'outer:`) for nested loops.

---

**Key points for speakers:**
- Labels start with a single quote (`'outer:`) — don't confuse with lifetime syntax (same notation, different context).
- Without labels, `break` and `continue` only affect the innermost loop.
- Labels are optional but essential for nested loop control.
- `break` can also return a value from a `loop` (covered in loop expressions).

**Common student questions:**
- *"Is `'outer` a lifetime?"* - No, it's a loop label. Same syntax, different meaning. Lifetimes appear in type signatures; loop labels appear before loops.
- *"Can I use any name for labels?"* - Yes, any valid identifier prefixed with `'`. Convention is lowercase like `'outer`, `'inner`, `'main_loop`.
- *"Does this exist in other languages?"* - Java has labeled break/continue. C doesn't (you'd use `goto`). Python doesn't have it.

**Demo suggestion:**
Ask students: "What would happen if we used `break` without `'outer`?" (Answer: only the inner loop would exit, and the outer loop would continue with the next value)

</details>
