# Where to Go Next

Congratulations! You've written your first bare metal Rust programs. Here are
some directions to continue your embedded journey.

## Try: Touch Sensing (No Extra Hardware!)

The micro:bit v2 logo has capacitive touch sensing built-in:

```rust,editable,compile_fail
#![no_main]
#![no_std]

extern crate panic_halt as _;

use cortex_m_rt::entry;
use microbit::Board;
use embedded_hal::digital::{InputPin, OutputPin};

#[entry]
fn main() -> ! {
    let mut board = Board::take().unwrap();

    // Configure logo pin for touch sensing
    let touch = board.pins.p1_04.into_floating_input();

    board.display_pins.col3.set_low().unwrap();

    loop {
        if touch.is_low().unwrap() {
            // Logo is being touched - light center LED
            board.display_pins.row3.set_high().unwrap();
        } else {
            board.display_pins.row3.set_low().unwrap();
        }
    }
}
```

Touch the gold logo and watch the LED respond!

## Built-in Features to Explore

The micro:bit v2 has many built-in peripherals you can experiment with:

| Feature | Description | Complexity |
|---------|-------------|------------|
| **Temperature sensor** | Built into the nRF52833 chip | Easy |
| **Accelerometer** | LSM303AGR via I2C | Medium |
| **Compass** | LSM303AGR magnetometer | Medium |
| **Speaker** | PWM-driven buzzer | Medium |
| **Microphone** | PDM digital microphone | Advanced |
| **Bluetooth** | BLE with SoftDevice | Advanced |

## Recommended Resources

### Books and Tutorials

- [Discovery Book (micro:bit v2)](https://docs.rust-embedded.org/discovery-mb2/) -
  Official Rust Embedded WG tutorial, comprehensive
- [impl Rust for Microbit](https://mb2.implrust.com/) -
  Project-based learning with fun examples
- [The Embedded Rust Book](https://docs.rust-embedded.org/book/) -
  General embedded Rust concepts

### Frameworks

- [Embassy](https://embassy.dev/) - Async embedded framework, great for
  complex applications with multiple tasks
- [RTIC](https://rtic.rs/) - Real-Time Interrupt-driven Concurrency framework

### Community

- [Rust Embedded Matrix Chat](https://matrix.to/#/#rust-embedded:matrix.org)
- [Rust Embedded GitHub](https://github.com/rust-embedded)
- [awesome-embedded-rust](https://github.com/rust-embedded/awesome-embedded-rust) -
  Curated list of crates and resources

## What's Different in Production Code?

As you progress, you'll encounter more advanced patterns:

| Learning Code | Production Code |
|---------------|-----------------|
| `delay()` busy-wait | Hardware timers, interrupts |
| Polling buttons | Interrupt-driven input |
| Blocking I/O | Async with Embassy |
| Manual row scanning | Display driver crates |
| `panic_halt` | `defmt` + RTT for debugging |

<details>

**Key points:**

- The micro:bit is a great platform to keep experimenting with
- Embassy makes complex async embedded code much easier
- The Discovery Book is the logical next step for deeper learning

**For the instructor:**

- Demo the touch sensing if time permits - it's a crowd-pleaser
- Point students to the Discovery Book for self-study
- Embassy is the future of embedded Rust but adds complexity

</details>
