# Palindrome

Implement a Rust function that determines whether a given string is a palindrome. 

A palindrome is a sequence of characters that reads the same forward and backward (e.g. kayak, madam, racecar, ...). 

In this exercices we ask you to ignore spaces, punctuation, and case.

Have a look at the [`std::str`](https://doc.rust-lang.org/std/primitive.str.html) documentation if needed.

Function signature:
```rs
fn is_palindrome(s: &str) -> bool{
    // Your code
}
```

Usage example:
```rs
fn main() {
    let palindrome1 = "A man, a plan, a canal, Panama";
    let palindrome2 = "Madam, in Eden, I'm Adam";
    let non_palindrome = "Having class 8:30am is fun!!";

    println!("Is '{}' a palindrome? {}", palindrome1, is_palindrome(palindrome1)); // True
    println!("Is '{}' a palindrome? {}", palindrome2, is_palindrome(palindrome2)); // True
    println!("Is '{}' a palindrome? {}", non_palindrome, is_palindrome(non_palindrome)); // False
}
```

<details>

**Exercise guidance for speakers:**
- This is a great introductory exercise for string manipulation.
- Students practice `chars()`, `filter()`, and iterator methods.
- Key insight: compare filtered chars with reversed filtered chars.

**Key concepts practiced:**
1. String methods: `chars()`, `to_lowercase()`.
2. Iterator methods: `filter()`, `rev()`, `eq()`.
3. Closures for filtering predicates.
4. Character methods: `is_alphanumeric()`.

**Hints to give if stuck:**
- Filter to keep only alphanumeric characters.
- Convert to lowercase for case-insensitive comparison.
- Use `rev()` to reverse an iterator.
- Compare with `eq()` or `collect()` and compare collections.

**Elegant solution:**
```rust
fn is_palindrome(s: &str) -> bool {
    let chars: Vec<_> = s.chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    chars.iter().eq(chars.iter().rev())
}
```

**Common mistakes:**
- Forgetting to handle case sensitivity.
- Not filtering out spaces and punctuation.
- Inefficient double iteration (can be done in one pass with `take(n/2)` comparison).

</details>