# LED Pattern Controller

Create a program that displays LED patterns and responds to button input.

## Requirements

1. **Display patterns** on the 5x5 LED matrix
2. **Button A** cycles through 3 different patterns
3. **Button B** toggles the display on/off

## Starter Code

Copy this to `src/exercises/course-8/src/bin/led-pattern.rs`:

```rust,compile_fail
#![no_main]
#![no_std]

extern crate panic_halt as _;
use cortex_m::asm::delay;
use cortex_m_rt::entry;
use microbit::Board;
use embedded_hal::digital::{InputPin, OutputPin};

// Simple 5x5 patterns (1 = LED on, 0 = LED off)
const PATTERN_HEART: [[u8; 5]; 5] = [
    [0, 1, 0, 1, 0],
    [1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1],
    [0, 1, 1, 1, 0],
    [0, 0, 1, 0, 0],
];

const PATTERN_SMILE: [[u8; 5]; 5] = [
    [0, 0, 0, 0, 0],
    [0, 1, 0, 1, 0],
    [0, 0, 0, 0, 0],
    [1, 0, 0, 0, 1],
    [0, 1, 1, 1, 0],
];

const PATTERN_CROSS: [[u8; 5]; 5] = [
    [1, 0, 0, 0, 1],
    [0, 1, 0, 1, 0],
    [0, 0, 1, 0, 0],
    [0, 1, 0, 1, 0],
    [1, 0, 0, 0, 1],
];

#[entry]
fn main() -> ! {
    let mut board = Board::take().unwrap();

    let mut current_pattern: usize = 0;
    let mut display_on: bool = true;
    let mut button_a_was_pressed = false;
    let mut button_b_was_pressed = false;

    loop {
        // TODO: Check button A for pattern change
        // Hint: Detect button press (transition from not pressed to pressed)
        // to avoid cycling through patterns while button is held
        let button_a_pressed = board.buttons.button_a.is_low().unwrap();

        // TODO: Implement pattern cycling logic here

        // TODO: Check button B for display toggle
        let button_b_pressed = board.buttons.button_b.is_low().unwrap();

        // TODO: Implement display toggle logic here

        // TODO: Display the current pattern if display_on is true
        // Hint: You'll need to iterate through rows and columns
        // For each LED position, check the pattern array

        // Update previous button states
        button_a_was_pressed = button_a_pressed;
        button_b_was_pressed = button_b_pressed;

        // Small delay for button debouncing
        delay(10_000);
    }
}

// Helper function to get the current pattern
fn get_pattern(index: usize) -> &'static [[u8; 5]; 5] {
    match index {
        0 => &PATTERN_HEART,
        1 => &PATTERN_SMILE,
        _ => &PATTERN_CROSS,
    }
}
```

## Hints

<details>
<summary>Hint 1: Button Edge Detection</summary>

To detect a button *press* (not just held), check for a transition:

```rust,ignore
if button_a_pressed && !button_a_was_pressed {
    // Button was just pressed
    current_pattern = (current_pattern + 1) % 3;
}
```

</details>

<details>
<summary>Hint 2: Displaying a Pattern</summary>

The LED matrix needs row scanning. A simplified approach for one row:

```rust,ignore
let pattern = get_pattern(current_pattern);

// Display row 0
if display_on && pattern[0][0] == 1 {
    board.display_pins.col1.set_low().unwrap();
} else {
    board.display_pins.col1.set_high().unwrap();
}
board.display_pins.row1.set_high().unwrap();
delay(1000);
board.display_pins.row1.set_low().unwrap();

// Repeat for other rows...
```

</details>

<details>
<summary>Hint 3: Full Display Loop</summary>

For a complete display, scan all rows rapidly:

```rust,ignore
let cols = [
    &mut board.display_pins.col1,
    &mut board.display_pins.col2,
    &mut board.display_pins.col3,
    &mut board.display_pins.col4,
    &mut board.display_pins.col5,
];

let rows = [
    &mut board.display_pins.row1,
    &mut board.display_pins.row2,
    &mut board.display_pins.row3,
    &mut board.display_pins.row4,
    &mut board.display_pins.row5,
];

// You'll need to work around the borrow checker here!
// Consider using the microbit::display module for easier access.
```

</details>

## Bonus Challenges

1. **Animation**: Make the pattern scroll or animate
2. **Brightness**: Use PWM to control LED brightness with buttons
3. **Sound**: Add a beep when changing patterns (use the speaker)

## Running the Exercise

```bash
cargo embed --bin led-pattern
```

<details>

**Solution approach:**

The key insight is that you need:
1. Edge detection for buttons (not level detection)
2. Row scanning for the LED matrix (or use a timer interrupt)
3. State variables that persist across loop iterations

**Why manual row scanning?**

We implement row scanning manually here to understand how LED matrices work
at a low level. In real projects, use the `Display` API which handles this
automatically:

```rust,ignore
use microbit::{display::blocking::Display, hal::Timer};

let mut display = Display::new(board.display_pins);
let mut timer = Timer::new(board.TIMER0);

// Much simpler! Handles row scanning internally
display.show(&mut timer, pattern, 1000);
```

</details>
