# Unwrap

In Rust, `.unwrap()` is a method used on types like `Option<T>` and `Result<T, E>` to extract the value inside when you are certain it exists (i.e., it is `Some(value)` or `Ok(value)`). 

However, if the value is `None` or `Err`, `.unwrap()` will panic (crash your program).

```rust,editable
fn main() {
    let some_value: Option<i32> = Some(42);
    let x = some_value.unwrap(); // x = 42
    println!("x: {x}");

    let none_value: Option<i32> = None;
    let y = none_value.unwrap(); // PANIC!
    println!("y: {y}");
}
```

```rust,editable
fn main() {
    let result: Result<i32, &str> = Ok(42);
    let x = result.unwrap(); // x = 42
    println!("x: {x}");

    let err_result: Result<i32, &str> = Err("Something went wrong");
    let y = err_result.unwrap(); // PANIC: "Something went wrong"
    println!("y: {y}");
}
```

This is [how cloudflare "crashed" 20% of the Internet](https://blog.cloudflare.com/18-november-2025-outage/).

Prefer explicit error handling in real-world code to avoid crashes.
* Use `.expect("message")` to provide a custom panic message.
* Use pattern matching (`match` or `if let`) for explicit error handling.
* Use methods like `.unwrap_or(default)` or `?` (the "try" operator) for safer handling.


<details>

 * `Result` is either `Ok` or `Err`.
 * We'll see `match` and `if let` later.
    
</details>
