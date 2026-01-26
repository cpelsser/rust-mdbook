# Blink an LED

Let's make something happen! We'll turn on an LED, then make it blink.

## Turn On a Single LED

The `microbit-v2` crate provides easy access to the board's hardware:

```rust,editable,compile_fail
#![no_main]
#![no_std]

extern crate panic_halt as _;
use cortex_m_rt::entry;
use microbit::Board;
use embedded_hal::digital::OutputPin;

#[entry]
fn main() -> ! {
    let mut board = Board::take().unwrap();

    // The LED matrix needs column LOW and row HIGH
    board.display_pins.col1.set_low().unwrap();
    board.display_pins.row1.set_high().unwrap();

    loop {}
}
```

Run this with:

```bash
cargo embed --bin blink
```

You should see the top-left LED light up!

## Understanding the Code

1. **`Board::take()`** - Gets ownership of all board peripherals (can only be
   called once)
2. **`display_pins`** - Access to the LED matrix pins
3. **`set_low()` / `set_high()`** - Control the pin voltage

## Adding a Delay

A static LED is boring. Let's make it blink using a simple delay:

```rust,editable,compile_fail
#![no_main]
#![no_std]

extern crate panic_halt as _;
use cortex_m::asm::delay;
use cortex_m_rt::entry;
use microbit::Board;
use embedded_hal::digital::OutputPin;

#[entry]
fn main() -> ! {
    let mut board = Board::take().unwrap();

    board.display_pins.col1.set_low().unwrap();

    loop {
        board.display_pins.row1.set_high().unwrap();
        delay(64_000_000);  // Approximately 1 second at 64MHz

        board.display_pins.row1.set_low().unwrap();
        delay(64_000_000);
    }
}
```

The `delay()` function burns CPU cycles - one cycle per count. The nRF52833
runs at 64 MHz (64 million cycles per second), so `delay(64_000_000)` waits
roughly 1 second. It's not precise due to loop overhead, but it works for
learning! For precise timing in real applications, use a hardware timer.

> **Note:** We use `cortex_m::asm::delay` here to demonstrate bare metal
> programming with minimal dependencies. For real projects, use the HAL's
> `Timer` with `DelayNs` trait for precise, portable timing:
> ```rust,ignore
> use embedded_hal::delay::DelayNs;
> use microbit::hal::Timer;
> let mut timer = Timer::new(board.TIMER0);
> timer.delay_ms(1000);  // Precise 1 second delay
> ```

## Try It Yourself

1. Change the delay to make it blink faster or slower
2. Light up a different LED (try `col2`/`row2`)
3. Create a pattern with multiple LEDs

<details>

**Key points:**

- `Board::take()` follows the singleton pattern - peripherals can only be
  taken once
- `embedded_hal::digital::OutputPin` is a trait - same code works on any chip
- `delay()` is a busy-wait loop, not a proper timer

**Common questions:**

- *"Why `unwrap()` everywhere?"* - These operations can't really fail on
  this hardware, but the traits return `Result` for portability.
- *"Is busy-waiting bad?"* - For blinking an LED, it's fine. For real
  applications, use hardware timers or interrupts.

</details>
