# Move Semantics

An assignment will transfer ownership between variables:

```rust,editable
fn main() {
    let s1: String = String::from("Hello!");
    let s2: String = s1;
    println!("s2: {s2}");
    // println!("s1: {s1}");
}
```

* The assignment of `s1` to `s2` transfers ownership.
* The data was _moved_ from `s1` and `s1` is no longer accessible.
* When `s1` goes out of scope, nothing happens: it has no ownership.
* When `s2` goes out of scope, the string data is freed.
* There is always _exactly_ one variable binding which owns a value.

<details>

**Key points for speakers:**
- Move semantics are Rust's default for non-Copy types — no special syntax needed.
- This is different from Python, where assignment creates a new reference to the same object.
- In Rust, clones are always explicit (by using `clone`), making performance costs visible.

**Comparison with Python:**
In Python, `s2 = s1` makes both variables point to the same object (shared reference).
In Rust, `let s2 = s1` *moves* ownership — only `s2` is valid after this.

</details>