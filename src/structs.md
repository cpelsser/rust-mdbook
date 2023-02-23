# Structs

Like C and C++, Rust has support for custom structs:

```rust,editable
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let mut peter = Person {
        name: String::from("Peter"),
        age: 27,
    };
    println!("{} is {} years old", peter.name, peter.age);
    
    peter.age = 28;
    println!("{} is {} years old", peter.name, peter.age);
    
    let jackie = Person {
        name: String::from("Jackie"),
        ..peter
    };
    println!("{} is {} years old", jackie.name, jackie.age);
}
```

<details>
Key Points: 

* Structs work like in C or C++.
  * Like in C++, and unlike in C, no typedef is needed to define a type.
  * Unlike in C++, there is no inheritance between structs.
* Methods are defined in an `impl` block, which we will see in following slides.
* There are different types of structs. 
  * Zero-sized structs `e.g., struct Foo;` `is a structure that does not require any memore. It might be used when implementing a trait on some type but don’t have any data that you want to store in the value itself. 
  * The next slide will introduce Tuple structs.
* .. is an operator that is used for pattern binding here. It performs a "and the rest" pattern binding.


</details>
