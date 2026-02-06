# `HashMap`

Standard hash map with protection against HashDoS attacks:

```rust,editable
use std::collections::HashMap;

fn main() {
    let mut page_counts = HashMap::new();
    page_counts.insert("Adventures of Huckleberry Finn".to_string(), 207);
    page_counts.insert("Grimms' Fairy Tales".to_string(), 751);
    page_counts.insert("Pride and Prejudice".to_string(), 303);

    if !page_counts.contains_key("Les Misérables") {
        println!("We know about {} books, but not Les Misérables.",
                 page_counts.len());
    }

    for book in ["Pride and Prejudice", "Alice's Adventure in Wonderland"] {
        match page_counts.get(book) {
            Some(count) => println!("{book}: {count} pages"),
            None => println!("{book} is unknown.")
        }
    }
}
```

<details>

- `HashMap` is not defined in the prelude and needs to be brought into scope.
- Try using `entry()` to insert a value if nothing is found:
  ```rust,ignore
  let pc = page_counts.entry("The Hunger Games".to_string()).or_insert(374);
  ```
- Unlike `vec!`, there is no standard `hashmap!` macro, but since Rust 1.56,
  HashMap implements `From<[(K, V); N]>`:
  ```rust,ignore
  let page_counts = HashMap::from([
      ("Harry Potter".to_string(), 336),
      ("The Hunger Games".to_string(), 374),
  ]);
  ```

</details>
