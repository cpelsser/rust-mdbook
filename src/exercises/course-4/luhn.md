# Luhn Algorithm

The [Luhn algorithm](https://en.wikipedia.org/wiki/Luhn_algorithm) is used to
validate credit card numbers. The algorithm takes a string as input and does the
following to validate the credit card number:

* Ignore all spaces. Reject number with less than two digits.

* Moving from right to left, double every second digit: for the number `1234`,
  we double `3` and `1`.

* After doubling a digit, sum the digits. So doubling `7` becomes `14` which
  becomes `5`.

* Sum all the undoubled and doubled digits.

* The credit card number is valid if the sum ends with `0`.

Copy the following code to <https://play.rust-lang.org/> and implement the
function:


```rust
// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

{{#include luhn.rs:luhn}}
    unimplemented!()
}

{{#include luhn.rs:unit-tests}}

#[allow(dead_code)]
fn main() {}
```

<details>

**Exercise guidance for speakers:**
- This exercise practices string processing, iteration, and the `?` operator.
- Encourage using iterator methods: `chars()`, `filter()`, `rev()`, `enumerate()`.
- The algorithm works right-to-left, so `rev()` is useful.
- Students often forget edge cases: all spaces, single digit, non-digit characters.

**Hints to give if stuck:**
1. First, filter out spaces and collect to a String or Vec.
2. Check minimum length (>= 2 digits).
3. Iterate right-to-left, doubling every other digit.
4. Sum digits (remember 14 → 1 + 4 = 5).
5. Check if sum % 10 == 0.

**Common student mistakes:**
- Forgetting to handle spaces.
- Doubling from left instead of right.
- Not handling the "sum digits of doubled value" step.
- Off-by-one errors in which digits to double.

**Solution approach:**
```rust
cc.chars()
  .filter(|c| !c.is_whitespace())
  .rev()
  .enumerate()
  .map(|(i, c)| { /* transform based on position */ })
  .sum::<u32>() % 10 == 0
```

</details>
