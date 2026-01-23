# Deriving Traits

You can let the compiler derive a number of traits:

```rust,editable
#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Player {
    name: String,
    strength: u8,
    hit_points: u8,
}

fn main() {
    let p1 = Player::default();
    let p2 = p1.clone();
    println!("Is {:?}\nequal to {:?}?\nThe answer is {}!", &p1, &p2,
             if p1 == p2 { "yes" } else { "no" });
}
```

<details>

- Deriving only works for traits that have standard implementations based on the
  struct's fields.
- Common derivable traits: `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`,
  `PartialOrd`, `Ord`, `Hash`, `Default`.
- You can only derive `Copy` if all fields implement `Copy`.
- For custom behavior, implement the trait manually instead of deriving.

</details>
