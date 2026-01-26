# Your First Bare Metal Program

Now let's write code! We'll start with the absolute minimum and build up to
blinking LEDs and reading buttons.

A bare metal Rust program has a different structure than regular Rust:

```rust,editable,compile_fail
#![no_main]
#![no_std]

extern crate panic_halt as _;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    loop {
        // Your code here
    }
}
```

## What's Different?

| Attribute | Meaning |
|-----------|---------|
| `#![no_main]` | We provide our own entry point, not the standard `main` |
| `#![no_std]` | Don't link the standard library |
| `-> !` | Function never returns (infinite loop) |
| `#[entry]` | Marks this as the program entry point |

## The Panic Handler

When something goes wrong (like `unwrap()` on `None`), Rust needs a panic
handler. The `panic_halt` crate provides a simple one that just halts the CPU:

```rust,ignore
extern crate panic_halt as _;  // Import for side effects (panic handler)
```

## Why `-> !`?

The `!` return type means "never returns". On bare metal, there's nowhere to
return to! Your program runs until:

- Power is removed
- The chip is reset
- It enters an infinite loop (intended or not)

This section covers:

- [Blink an LED](blink.md) - Output: controlling hardware
- [Reading Button Input](buttons.md) - Input: responding to the world

<details>

**Key points:**

- Every bare metal program needs `#![no_std]` and `#![no_main]`
- The panic handler is required - `panic_halt` is simplest
- The main loop runs forever - that's normal!

**Why `extern crate` instead of `use`?**

```rust,ignore
extern crate panic_halt as _;  // Traditional embedded style
use panic_halt as _;           // Also works in Rust 2018+
```

Both work, but `extern crate` is more explicit about intent: "link this crate
for its side effects (the panic handler), don't import anything into scope."

The `panic_halt` crate provides a `#[panic_handler]` function. We don't call
any functions from it - we just need it linked into our binary. The `as _`
means "import nothing into the namespace."

**Common student questions:**

- *"What happens after `main` returns?"* - It can't return! The `-> !` type
  enforces this at compile time.
- *"Why `as _`?"* - We need the crate linked (for its panic handler) but
  don't need to use any of its items by name.
- *"What does `panic_halt` actually do?"* - It defines a function that runs
  when a panic occurs. It simply enters an infinite loop, halting the CPU.
- *"Are there other panic handlers?"* - Yes! `panic-probe` prints via the
  debugger, `panic-rtt-target` uses RTT. For learning, `panic_halt` is simplest.

</details>
