# Meet the micro:bit

The [BBC micro:bit](https://microbit.org/) is a small computer designed for
education. Version 2 is perfect for learning embedded Rust because:

- **Built-in peripherals** - LEDs, buttons, sensors with no wiring
- **On-board debugger** - Flash and debug with just a USB cable
- **Great Rust support** - Well-maintained `microbit-v2` crate
- **Affordable** - Around $15-20 USD

This section covers:

- [Hardware Overview](microbit-hardware.md) - What's on the board
- [Development Setup](setup.md) - Installing the toolchain

<details>

**Key points:**

- The micro:bit v2 uses a Nordic nRF52833 chip (ARM Cortex-M4)
- No extra hardware (JTAG debugger, wires) needed
- Originally designed for BBC's "Make It Digital" campaign in the UK

**Common student questions:**

- *"Can I use micro:bit v1?"* - This course targets v2. The v1 has a different chip
  (nRF51822) and less memory. Code won't be directly compatible.
- *"Where can I buy one?"* - Official resellers at microbit.org, or Amazon/electronics
  retailers. Around $15-20 USD.
- *"What if I break it?"* - The micro:bit is quite robust. The most common issue is
  using a charge-only USB cable that doesn't transfer data.

</details>
