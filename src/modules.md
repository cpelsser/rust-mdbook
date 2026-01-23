# Modules

We have seen how `impl` blocks let us namespace functions to a type.

Similarly, `mod` lets us namespace types and functions:

```rust,editable
mod foo {
    pub fn do_something() {
        println!("In the foo module");
    }
}

mod bar {
    pub fn do_something() {
        println!("In the bar module");
    }
}

fn main() {
    foo::do_something();
    bar::do_something();
}
```

<details>

- Modules are used to organize code and control visibility (public/private).
- By default, items in a module are private to the parent module.
- Use `pub` to make items visible outside their module.
- Modules can be nested arbitrarily deep.

</details>
