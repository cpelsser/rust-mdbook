# Exercise: Log Filter

Implement a `Filter` that uses a closure to filter log messages, sending those
that pass the filtering predicate to an inner logger.

```rust,editable
pub trait Logger {
    /// Log a message at the given verbosity level.
    fn log(&self, verbosity: u8, message: &str);
}

struct StderrLogger;

impl Logger for StderrLogger {
    fn log(&self, verbosity: u8, message: &str) {
        eprintln!("verbosity={verbosity}: {message}");
    }
}

// TODO: Define and implement `Filter`.
// It should:
// - Store an inner logger
// - Store a predicate closure that takes (verbosity, message) and returns bool
// - Implement Logger, only forwarding messages where the predicate returns true

fn main() {
    let logger = Filter::new(StderrLogger, |_verbosity, msg| msg.contains("yikes"));
    logger.log(5, "FYI");
    logger.log(1, "yikes, something went wrong");
    logger.log(2, "uhoh");
}
```

<details>
<summary>Hints</summary>

- `Filter` needs two generic parameters: one for the inner logger type, and one
  for the predicate closure type.
- The predicate should be `Fn(u8, &str) -> bool`.
- Remember to add trait bounds in both the `impl` block and the trait implementation.
- Call the predicate with `(self.predicate)(verbosity, message)`.

</details>
