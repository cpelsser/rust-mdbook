# Match Guards

When matching, you can add a _guard_ to a pattern. This is an arbitrary Boolean
expression which will be executed if the pattern matches:

```rust,editable
{{#include ../../third_party/rust-by-example/match-guards.rs}}
```

<details>

Key Points:
* Match guards as a separate syntax feature are important and necessary.
* They are not the same as separate `if` expression inside of the match arm. An `if` expression inside of the branch block (after `=>`) happens after the match arm is selected. Failing the `if` condition inside of that block won't result in other arms
of the original `match` expression being considered.  
* You can use the variables defined in the pattern in your if expression.
* The condition defined in the guard applies to every expression in a pattern with an `|`.

```rust,editable
#[rustfmt::skip]
fn main() {
    let pair = (1, -2);
    println!("Tell me about {pair:?}");
    match pair {
        (x, y) if x == y     => println!("These are twins"),
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) | (x, -2) if x % 2 == 1 => println!("The first one is odd"),
        _                    => println!("No correlation..."),
    }
}
```
</details>
