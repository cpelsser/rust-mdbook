# Welcome to Bare Metal Rust

In this course, we'll explore *bare metal* programming: running Rust code directly
on hardware without an operating system. This is how firmware, bootloaders, and
embedded systems are built.

## What We'll Learn

- What `no_std` means and what changes without an OS
- The embedded Rust ecosystem (PACs, HALs, BSPs)
- How to program the BBC micro:bit v2
- Reading inputs and controlling outputs

## What We'll Build

By the end of this session, you'll have a working program that:

1. Lights up LEDs on the micro:bit display
2. Responds to button presses
3. Cycles through different LED patterns

## Why Rust for Embedded?

Rust is increasingly popular for embedded development because:

- **Memory safety without garbage collection** - Critical for resource-constrained devices
- **Zero-cost abstractions** - High-level code compiles to efficient machine code
- **No runtime** - Rust can run on the bare metal
- **Strong type system** - Catch hardware configuration errors at compile time

## Prerequisites

- Completed Courses 1-7 (especially ownership and traits)
- BBC micro:bit v2 hardware
- Development tools installed (see [Setup](bare-metal/setup.md))

<details>

**Key points:**

- Bare metal means no OS - your code is the only thing running
- Embedded Rust has grown significantly since 2018
- The micro:bit is ideal for learning: built-in LEDs, buttons, and debugger

**Common student questions:**

- *"Is this harder than regular Rust?"* - Different, not harder. Less library support,
  but the core language is the same.
- *"Do I need electronics experience?"* - No! The micro:bit requires no wiring.
- *"Will this work on other microcontrollers?"* - The concepts transfer! The specific
  crates differ (e.g., `esp-hal` for ESP32), but `embedded-hal` traits are universal.

</details>
