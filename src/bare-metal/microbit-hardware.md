# Hardware Overview

The micro:bit v2 packs many features into a small board:

```bob
                      "micro:bit v2"

      .------------------------------------------.
      |                                          |
      |  +---+     O   O   O   O   O      +---+  |
      |  | A |                            | B |  |
      |  +---+     O   O   O   O   O      +---+  |
      |                                          |
      |            O   O   O   O   O             |
      |                                          |
      |            O   O   O   O   O             |
      |                                          |
      |            O   O   O   O   O             |
      |                                          |
      |             +-------------+              |
      |             |     USB     |              |
      '------------------------------------------'
                     5x5 LED matrix
```

## Main Components

| Component | Description |
|-----------|-------------|
| **CPU** | Nordic nRF52833 (ARM Cortex-M4, 64MHz) |
| **Memory** | 128KB RAM, 512KB Flash |
| **LEDs** | 5x5 red LED matrix |
| **Buttons** | A (left) and B (right) |
| **Accelerometer** | LSM303AGR (motion sensing) |
| **Compass** | LSM303AGR (magnetometer) |
| **Speaker** | Built-in buzzer |
| **Microphone** | MEMS microphone |
| **Radio** | Bluetooth Low Energy, proprietary radio |

## The LED Matrix

The 5x5 LED display is controlled by a matrix of rows and columns:

- 5 row pins + 5 column pins = 10 GPIO pins (not 25!)
- To light an LED: set its row HIGH and column LOW
- Only one row can be active at a time (multiplexing)

For this course, we'll use the `microbit-v2` crate which handles this
complexity for us.

## The Buttons

Two buttons on the front:

- **Button A** - Connected to GPIO P0.14
- **Button B** - Connected to GPIO P0.23

Both are *active low*: pressing the button connects to ground (reads LOW).

<details>

**Key points:**

- The LED matrix uses multiplexing - advanced topic we'll skip
- Buttons are "active low" - pressed = logic 0
- The on-board debugger appears as a USB drive for easy flashing

**Common student questions:**

- *"Why active low?"* - It's a common hardware design pattern that
  provides better noise immunity.
- *"Why only 10 pins for 25 LEDs?"* - Multiplexing! By rapidly switching rows,
  we create the illusion of all LEDs being on simultaneously.
- *"What's GPIO?"* - General Purpose Input/Output. Pins that can be configured
  as either inputs (read signals) or outputs (send signals).

</details>
