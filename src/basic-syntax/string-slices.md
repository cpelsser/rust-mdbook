# `String` vs `str`

We can now understand the two string types in Rust:

```rust,editable
fn main() {
    let s1: &str = "World";
    println!("s1: {s1}");

    let mut s2: String = String::from("Hello ");
    println!("s2: {s2}");
    s2.push_str(s1);
    println!("s2: {s2}");
    
    let s3: &str = &s2[6..];
    println!("s3: {s3}");
}
```

Rust terminology:

* `&str` an immutable reference to a string slice.
* `String` a mutable string buffer.

<details>

* `&str` introduces a string slice, which is an immutable reference to UTF-8 encoded string data 
  stored in a block of memory. String literals (`”Hello”`), are stored in the program’s binary.

* Rust’s `String` type is a wrapper around a vector of bytes. As with a `Vec<T>`, it is owned.
  `Vec<T>` is an array that changes size dynamically.
    
* As with many other types `String::from()` creates a string from a string literal; `String::new()` 
  creates a new empty string, to which string data can be added using the `push()` and `push_str()` methods.

* The `format!()` macro is a convenient way to generate an owned string from dynamic values. It 
  accepts the same format specification as `println!()`.
    
* You can borrow `&str` slices from `String` via `&` and optionally range selection.
    
* For C programmers: think of `&str` as `const char*` from C, but one that always points
  to a valid string in memory and knows its length (no null terminator needed).

* For Python programmers: `&str` is like an immutable Python string, while `String` is like
  a mutable string buffer. The key difference from Python is that Rust distinguishes between
  owned strings (`String`) and borrowed views into strings (`&str`).

* How do I modify s1? Here is an example built by iteratively following the guidelines of the compiler. 
  ```rust,editable
  fn main() {
      let mut s1: &str = "World";
      println!("s1: {s1}");
      let binding  = &(s1.to_owned() + " hobbit");
      s1 = &binding;

      let mut s2: String = String::from("Hello ");
      println!("s2: {s2}");
      s2.push_str(s1);
      println!("s2: {s2}");
    
      let s3: &str = &s2[6..];
      println!("s3: {s3}");
  }
  ```

  ```   
  fn to_owned(&self) -> Self::Owned
  ```
  Creates owned data from borrowed data, usually by cloning.
</details>
