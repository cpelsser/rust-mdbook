# Pattern Matching

The `match` keyword let you match a value against one or more _patterns_. The
comparisons are done from top to bottom and the first match wins.

The patterns can be simple values, similarly to `switch` in C (or `match` in Python 3.10+):

```rust,editable
fn main() {
    let input = 'x';

    match input {
        'q'                   => println!("Quitting"),
        'a' | 's' | 'w' | 'd' => println!("Moving around"),
        '0'..='9'             => println!("Number input"),
        _                     => println!("Something else"),
    }
}
```

The `_` pattern is a wildcard pattern which matches any value.

<details>
    
Key Points:
* You might point out how some specific characters are being used when in a patten
  * `|` as an `or`
  * `..` can expand as much as it needs to be
  * `1..=5` represents an inclusive range
  * `_` is a wild card
* It can be useful to show how binding works, by for instance replacing a wildcard character with a variable, or removing the quotes around `q`.
* You can demonstrate matching on a reference.
* This might be a good time to bring up the concept of irrefutable patterns, as the term can show up in error messages.

From [Rust Book](https://doc.rust-lang.org/book/ch18-02-refutability.html)

"Patterns that will match for any possible value passed are irrefutable. An example would be x in the statement let x = 5; because x matches anything and therefore cannot fail to match. Patterns that can fail to match for some possible value are refutable. An example would be Some(x) in the expression if let Some(x) = a_value because if the value in the a_value variable is None rather than Some, the Some(x) pattern will not match."
   
</details>
