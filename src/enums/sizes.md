# Enum Sizes

Rust enums are packed tightly, taking constraints due to alignment into account:

```rust,editable
use std::mem::{align_of, size_of};

macro_rules! dbg_size {
    ($t:ty) => {
        println!("{}: size {} bytes, align: {} bytes",
                 stringify!($t), size_of::<$t>(), align_of::<$t>());
    };
}

enum Foo {
    A,
    B,
}

#[repr(u32)]
enum Bar {
    A,  // 0
    B = 10000,
    C,  // 10001
}

fn main() {
    dbg_size!(Foo);
    dbg_size!(Bar);
    dbg_size!(bool);
    dbg_size!(Option<bool>);
    dbg_size!(&i32);
    dbg_size!(Option<&i32>);
    // dbg_size!(Option<&Foo>);
    // dbg_size!(&Foo);
}
```

* See the [Rust Reference](https://doc.rust-lang.org/reference/type-layout.html).

<details>
    
Key Points: 
 * Internally Rust is using a field (discriminant) to keep track of the enum variant.
 * `Bar` enum demonstrates that there is a way to control the discriminant value and type. If `repr` is removed, the discriminant type takes 2 bytes, becuase 10001 fits 2 bytes.
 * As a niche optimization an enum discriminant is merged with the pointer so that `Option<&Foo>` is the same size as `&Foo`.
 * `Option<bool>` is another example of tight packing.
 * For [some types](https://doc.rust-lang.org/std/option/#representation), Rust guarantees that `size_of::<T>()` equals `size_of::<Option<T>>()`.
 * Zero-sized types allow for efficient implementation of `HashSet` using `HashMap` with `()` as the value.
 * The ty module defines how rustc represents types internally. https://rustc-dev-guide.rust-lang.org/ty.html

</details>
