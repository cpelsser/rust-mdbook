# Field Shorthand Syntax

If you already have variables with the right names, then you can create the
struct using a shorthand:

```rust,editable
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: String, age: u8) -> Person {
        Person { name, age }
    }
}

fn main() {
    let peter = Person::new(String::from("Peter"), 27);
    println!("{peter:?}");
}
```

<details>

`name` and `age` are field names of the structure and variable names.

The `new` function could be written using `Self` as a type, as it is interchangeable with the struct type name

```rust,ignore
impl Person {
    fn new(name: String, age: u8) -> Self {
        Self { name, age }
    }
}
```
   
</details>
