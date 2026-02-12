# Static and Constant Variables

Global state is managed with static and constant variables.

## `const`

You can declare compile-time constants:

```rust,editable
const DIGEST_SIZE: usize = 3;
const ZERO: Option<u8> = Some(42);

fn compute_digest(text: &str) -> [u8; DIGEST_SIZE] {

    // println!("{:?}", text.as_bytes());
    // println!("{}", ZERO.unwrap_or(0));

    let mut digest = [ZERO.unwrap_or(0); DIGEST_SIZE];

    // println!("{:?}", digest);

    for (idx, &b) in text.as_bytes().iter().enumerate() {

        // println!("[{:?}]: {:?}", idx, &b);
        
        digest[idx % DIGEST_SIZE] = digest[idx % DIGEST_SIZE].wrapping_add(b);
    }
    digest
}

fn main() {
    let digest = compute_digest("Hello");
    println!("Digest: {digest:?}");
}
```

According the the [Rust RFC Book][1] these are inlined upon use.

## `static`

You can also declare static variables:

```rust,editable
static BANNER: &str = "Welcome to RustOS 3.14";

fn main() {
    println!("{BANNER}");
}
```

As noted in the [Rust RFC Book][1], these are not inlined upon use and have an actual associated memory location.  This is useful for unsafe and embedded code, and the variable lives through the entirety of the program execution.


We will look at mutating static data in the [chapter on Unsafe Rust](../unsafe.md).

<details>

* `pub fn unwrap_or(self, default: T) -> T`. Returns the contained `Some` value or a provided default. Here, ZERO.unwrap_or(0) is equal to 42. It would return 0 if ZERO was equal to None.
* `const` is used to define a constant value that is inlined wherever it is used. Inlining means that the compiler replaces all instances of the constant with its value.
* `static` on the other hand have a fixed memory location and can be mutated.

</details>

[1]: https://rust-lang.github.io/rfcs/0246-const-vs-static.html
