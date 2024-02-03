fn is_palindrome1(s: &str) -> bool {
    let clean_string: String = s.chars().filter(|c| c.is_alphanumeric()).collect();
    let reversed_string: String = clean_string.chars().rev().collect();

    clean_string.eq_ignore_ascii_case(&reversed_string)
}

// another possible solution, avoiding eq_ignore_ascii_case
fn is_palindrome2(s: &str) -> bool {
    let clean_string: String = s.chars().filter(|c| c.is_alphanumeric()).collect();
    let reversed_string: String = clean_string.chars().rev().collect();

    clean_string
        .chars()
        .zip(reversed_string.chars())
        .all(|(c1, c2)| c1.to_lowercase().next().unwrap() == c2.to_lowercase().next().unwrap())
}

fn main() {
    let palindrome1 = "A man, a plan, a canal, Panama";
    let palindrome2 = "Madam, in Eden, I'm Adam";
    let non_palindrome = "Rust is awesome";

    println!("Is '{}' a palindrome? {}", palindrome1, is_palindrome2(palindrome1));
    println!("Is '{}' a palindrome? {}", palindrome2, is_palindrome2(palindrome2));
    println!("Is '{}' a palindrome? {}", non_palindrome, is_palindrome2(non_palindrome));
}
