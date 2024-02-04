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