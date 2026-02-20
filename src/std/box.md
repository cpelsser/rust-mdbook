# `Box`

[`Box`][1] is an owned pointer to data on the heap:

```rust,editable
fn main() {
    let five = Box::new(5);
    println!("five: {}", *five);
}
```


```bob
 Stack                     Heap
.- - - - - - -.     .- - - - - - -.
:             :     :             :
:    five     :     :             :
:   +-----+   :     :   +-----+   :
:   | o---|---+-----+-->|  5  |   :
:   +-----+   :     :   +-----+   :
:             :     :             :
:             :     :             :
`- - - - - - -'     `- - - - - - -'
```

`Box<T>` implements `Deref<Target = T>`, which means that you can [call methods
from `T` directly on a `Box<T>`][2].

[1]: https://doc.rust-lang.org/std/boxed/struct.Box.html
[2]: https://doc.rust-lang.org/std/ops/trait.Deref.html#more-on-deref-coercion

<details>

* `Box` is like `std::unique_ptr` in C++.
  * Ownership: owns the value/object it points to.
  * Only one `Box`, one `unique_ptr` can own the data at a time.
  * Automatically deallocates memory when dropped/destroyed.
  <!--* Cannot be copied (only moved).-->
* In the above example, you can even leave out the `*` in the `println!` statement thanks to `Deref`.

</details>
