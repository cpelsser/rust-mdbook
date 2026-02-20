# Modules

We have seen how `impl` blocks let us namespace functions to a type.

`mod` is about organization: how your code is structured and accessed.

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
