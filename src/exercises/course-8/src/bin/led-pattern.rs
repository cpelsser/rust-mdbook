//! LED Pattern Controller Exercise
//!
//! Build a program that displays LED patterns and responds to button input.
//!
//! Requirements:
//! 1. Display patterns on the 5x5 LED matrix
//! 2. Button A cycles through 3 different patterns
//! 3. Button B toggles the display on/off

#![no_main]
#![no_std]

use cortex_m::asm::delay;
use cortex_m_rt::entry;
use embedded_hal::digital::{InputPin, OutputPin};
use microbit::Board;
extern crate panic_halt as _;

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

const PATTERNS: [&[[u8; 5]; 5]; 3] = [&PATTERN_HEART, &PATTERN_SMILE, &PATTERN_CROSS];

#[entry]
fn main() -> ! {
    let mut board = Board::take().unwrap();

    let mut current_pattern: usize = 0;
    let mut display_on: bool = true;
    let mut button_a_was_pressed = false;
    let mut button_b_was_pressed = false;

    loop {
        // Check button A for pattern change
        let button_a_pressed = board.buttons.button_a.is_low().unwrap();
        if button_a_pressed && !button_a_was_pressed {
            // Button A just pressed - cycle to next pattern
            current_pattern = (current_pattern + 1) % PATTERNS.len();
        }

        // Check button B for display toggle
        let button_b_pressed = board.buttons.button_b.is_low().unwrap();
        if button_b_pressed && !button_b_was_pressed {
            // Button B just pressed - toggle display
            display_on = !display_on;
        }

        // Display the current pattern using row scanning
        // Each row is briefly activated to create the full 5x5 image
        if display_on {
            let pattern = PATTERNS[current_pattern];

            // Row 1
            if pattern[0][0] == 1 {
                board.display_pins.col1.set_low().unwrap();
            } else {
                board.display_pins.col1.set_high().unwrap();
            }
            if pattern[0][1] == 1 {
                board.display_pins.col2.set_low().unwrap();
            } else {
                board.display_pins.col2.set_high().unwrap();
            }
            if pattern[0][2] == 1 {
                board.display_pins.col3.set_low().unwrap();
            } else {
                board.display_pins.col3.set_high().unwrap();
            }
            if pattern[0][3] == 1 {
                board.display_pins.col4.set_low().unwrap();
            } else {
                board.display_pins.col4.set_high().unwrap();
            }
            if pattern[0][4] == 1 {
                board.display_pins.col5.set_low().unwrap();
            } else {
                board.display_pins.col5.set_high().unwrap();
            }
            board.display_pins.row1.set_high().unwrap();
            delay(2000);
            board.display_pins.row1.set_low().unwrap();

            // Row 2
            if pattern[1][0] == 1 {
                board.display_pins.col1.set_low().unwrap();
            } else {
                board.display_pins.col1.set_high().unwrap();
            }
            if pattern[1][1] == 1 {
                board.display_pins.col2.set_low().unwrap();
            } else {
                board.display_pins.col2.set_high().unwrap();
            }
            if pattern[1][2] == 1 {
                board.display_pins.col3.set_low().unwrap();
            } else {
                board.display_pins.col3.set_high().unwrap();
            }
            if pattern[1][3] == 1 {
                board.display_pins.col4.set_low().unwrap();
            } else {
                board.display_pins.col4.set_high().unwrap();
            }
            if pattern[1][4] == 1 {
                board.display_pins.col5.set_low().unwrap();
            } else {
                board.display_pins.col5.set_high().unwrap();
            }
            board.display_pins.row2.set_high().unwrap();
            delay(2000);
            board.display_pins.row2.set_low().unwrap();

            // Row 3
            if pattern[2][0] == 1 {
                board.display_pins.col1.set_low().unwrap();
            } else {
                board.display_pins.col1.set_high().unwrap();
            }
            if pattern[2][1] == 1 {
                board.display_pins.col2.set_low().unwrap();
            } else {
                board.display_pins.col2.set_high().unwrap();
            }
            if pattern[2][2] == 1 {
                board.display_pins.col3.set_low().unwrap();
            } else {
                board.display_pins.col3.set_high().unwrap();
            }
            if pattern[2][3] == 1 {
                board.display_pins.col4.set_low().unwrap();
            } else {
                board.display_pins.col4.set_high().unwrap();
            }
            if pattern[2][4] == 1 {
                board.display_pins.col5.set_low().unwrap();
            } else {
                board.display_pins.col5.set_high().unwrap();
            }
            board.display_pins.row3.set_high().unwrap();
            delay(2000);
            board.display_pins.row3.set_low().unwrap();

            // Row 4
            if pattern[3][0] == 1 {
                board.display_pins.col1.set_low().unwrap();
            } else {
                board.display_pins.col1.set_high().unwrap();
            }
            if pattern[3][1] == 1 {
                board.display_pins.col2.set_low().unwrap();
            } else {
                board.display_pins.col2.set_high().unwrap();
            }
            if pattern[3][2] == 1 {
                board.display_pins.col3.set_low().unwrap();
            } else {
                board.display_pins.col3.set_high().unwrap();
            }
            if pattern[3][3] == 1 {
                board.display_pins.col4.set_low().unwrap();
            } else {
                board.display_pins.col4.set_high().unwrap();
            }
            if pattern[3][4] == 1 {
                board.display_pins.col5.set_low().unwrap();
            } else {
                board.display_pins.col5.set_high().unwrap();
            }
            board.display_pins.row4.set_high().unwrap();
            delay(2000);
            board.display_pins.row4.set_low().unwrap();

            // Row 5
            if pattern[4][0] == 1 {
                board.display_pins.col1.set_low().unwrap();
            } else {
                board.display_pins.col1.set_high().unwrap();
            }
            if pattern[4][1] == 1 {
                board.display_pins.col2.set_low().unwrap();
            } else {
                board.display_pins.col2.set_high().unwrap();
            }
            if pattern[4][2] == 1 {
                board.display_pins.col3.set_low().unwrap();
            } else {
                board.display_pins.col3.set_high().unwrap();
            }
            if pattern[4][3] == 1 {
                board.display_pins.col4.set_low().unwrap();
            } else {
                board.display_pins.col4.set_high().unwrap();
            }
            if pattern[4][4] == 1 {
                board.display_pins.col5.set_low().unwrap();
            } else {
                board.display_pins.col5.set_high().unwrap();
            }
            board.display_pins.row5.set_high().unwrap();
            delay(2000);
            board.display_pins.row5.set_low().unwrap();
        } else {
            // Display off - all LEDs off
            board.display_pins.col1.set_high().unwrap();
            board.display_pins.col2.set_high().unwrap();
            board.display_pins.col3.set_high().unwrap();
            board.display_pins.col4.set_high().unwrap();
            board.display_pins.col5.set_high().unwrap();
            delay(10000);
        }

        // Update previous button states
        button_a_was_pressed = button_a_pressed;
        button_b_was_pressed = button_b_pressed;
    }
}
