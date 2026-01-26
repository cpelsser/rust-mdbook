# The Embedded Ecosystem

Rust's embedded ecosystem is organized in layers, from low-level hardware access
to high-level board support:

```bob
    .------------------------------------.
    |      Your Application Code         |
    +------------------------------------+
    |    BSP (Board Support Package)     |
    |          "microbit-v2"             |
    +------------------------------------+
    |     HAL (Hardware Abstraction)     |
    |          "nrf52833-hal"            |
    +------------------------------------+
    |      PAC (Peripheral Access)       |
    |          "nrf52833-pac"            |
    +------------------------------------+
    |             Hardware               |
    |        Nordic nRF52833 chip        |
    '------------------------------------'
```

## PAC - Peripheral Access Crate

The lowest level. Auto-generated from vendor SVD files, provides raw register access:

```rust,ignore
// Direct register manipulation - verbose and error-prone
let peripherals = pac::Peripherals::take().unwrap();
peripherals.P0.pin_cnf[21].write(|w| w.dir().output());
peripherals.P0.outset.write(|w| w.pin21().set_bit());
```

## HAL - Hardware Abstraction Layer

Wraps the PAC with safe, idiomatic Rust APIs:

```rust,ignore
// HAL provides typed, safe access
let gpio = p0::Parts::new(peripherals.P0);
let mut led = gpio.p0_21.into_push_pull_output(Level::Low);
led.set_high().unwrap();
```

## BSP - Board Support Package

Provides board-specific conveniences and pin mappings:

```rust,ignore
// BSP knows about the board layout
let board = Board::take().unwrap();
board.display_pins.row1.set_high().unwrap();
```

## `embedded-hal` Traits

The [`embedded-hal`](https://crates.io/crates/embedded-hal) crate defines
standard traits that all HALs implement:

- `InputPin`, `OutputPin` - Digital I/O
- `DelayNs` - Timing delays
- `I2c`, `Spi` - Communication buses

This allows writing **portable drivers** that work on any microcontroller!

<details>

**Key points:**

- Start with BSP for simplicity, drop to HAL/PAC when needed
- `embedded-hal` traits enable code reuse across platforms
- The micro:bit BSP (`microbit-v2`) wraps `nrf52833-hal`

**Common questions:**

- *"Which layer should I use?"* - Start with BSP. Use HAL for features
  the BSP doesn't expose. Use PAC only for advanced/undocumented features.

</details>
