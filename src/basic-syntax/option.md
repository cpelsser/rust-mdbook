
# Option

Option<type> comes from `std::option`. It is used to represent an optional value. Option is either `Some` or `None`. 

```rust,editable
fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

fn main () {
    // The return value of the function is an option
    let result = divide(2.0, 3.0);

    // Pattern match to retrieve the value
    match result {
        // The division was valid
        Some(x) => println!("Result: {x}"),
        // The division was invalid
        None    => println!("Cannot divide by 0"),
    }
}
```

<details>

* See [std::option](https://doc.rust-lang.org/std/option/) for more info.

</details>