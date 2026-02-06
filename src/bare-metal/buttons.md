# Reading Button Input

Now let's respond to the real world by reading button presses.

## Basic Button Reading

The buttons are inputs, so we use the `InputPin` trait:

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

    // Set up one LED
    board.display_pins.col1.set_low().unwrap();

    loop {
        // Button is "active low" - pressed = is_low()
        if board.buttons.button_a.is_low().unwrap() {
            board.display_pins.row1.set_high().unwrap();
        } else {
            board.display_pins.row1.set_low().unwrap();
        }
    }
}
```

Press button A and the LED lights up!

## Active Low Buttons

The micro:bit buttons are *active low*:

| Button State | `is_low()` | `is_high()` |
|--------------|------------|-------------|
| Pressed | `true` | `false` |
| Released | `false` | `true` |

This is a common hardware design - when not pressed, the pin is pulled high
internally.

## Using Both Buttons

Let's use button A and button B for different actions:

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

    // Set up two columns of LEDs
    board.display_pins.col1.set_low().unwrap();
    board.display_pins.col5.set_low().unwrap();

    loop {
        // Button A controls left LED
        if board.buttons.button_a.is_low().unwrap() {
            board.display_pins.row1.set_high().unwrap();
        } else {
            board.display_pins.row1.set_low().unwrap();
        }

        // Button B controls right LED
        if board.buttons.button_b.is_low().unwrap() {
            board.display_pins.row5.set_high().unwrap();
        } else {
            board.display_pins.row5.set_low().unwrap();
        }
    }
}
```

## Button Debouncing

Real buttons "bounce" - they rapidly switch on/off when pressed. For this
course, we'll keep it simple. In production code, you'd add debouncing logic.

<details>

**Key points:**

- Buttons are inputs - use `InputPin` trait
- Active low is common in hardware - pressed means LOW
- The main loop polls continuously - works but uses CPU

**Common student questions:**

- *"Why active low?"* - Better noise immunity and simpler circuit design.
  The internal pull-up resistor keeps the pin high until grounded by the button.
- *"What about interrupts?"* - You can trigger code on button press without
  polling, but that's an advanced topic.
- *"What is debouncing?"* - Physical buttons bounce (rapidly connect/disconnect)
  for a few milliseconds. Debouncing filters this noise, typically by waiting
  ~10-50ms after detecting a press before reading again.
- *"Why does my button seem stuck?"* - Make sure you're checking inside the loop,
  not just once at startup. The loop runs continuously to poll the button state.

**Demo suggestion:**

Show what happens without debouncing by counting button presses - you'll
often get multiple counts per press.

</details>
