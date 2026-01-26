# Development Setup

Before the session, install the required tools. This takes 10-15 minutes.

## 1. Install the ARM Target

The micro:bit uses an ARM Cortex-M4 processor:

```bash
rustup target add thumbv7em-none-eabihf
```

## 2. Install probe-rs

[probe-rs](https://probe.rs/) is a modern tool for flashing and debugging
embedded devices:

**Linux/macOS:**
```bash
curl --proto '=https' --tlsv1.2 -LsSf \
  https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh
```

**Windows:**
```powershell
irm https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.ps1 | iex
```

## 3. USB Permissions (Linux only)

On Linux, add a udev rule to access the micro:bit without root:

```bash
echo 'SUBSYSTEM=="hidraw", ATTRS{idVendor}=="0d28", MODE="0660", GROUP="plugdev", TAG+="uaccess"' | \
  sudo tee /etc/udev/rules.d/50-microbit.rules
sudo udevadm control --reload-rules
```

Then log out and back in, or run `sudo udevadm trigger`.

## 4. Verify the Setup

Connect your micro:bit via USB and run:

```bash
probe-rs list
```

You should see something like:

```text
The following debug probes were found:
[0]: BBC micro:bit CMSIS-DAP -- 0d28:0204:990636020005282030f1... (CMSIS-DAP)
```

## Troubleshooting

| Problem | Solution |
|---------|----------|
| `probe-rs list` shows nothing | Try a different USB cable (some are charge-only) |
| Permission denied (Linux) | Check udev rules, logout/login |
| Device not found (macOS) | No special drivers needed, try different port |
| Device not found (Windows) | Install [Zadig](https://zadig.akeo.ie/) driver |

<details>

**Key points:**

- `thumbv7em-none-eabihf` = Thumb instruction set, Cortex-M4, no OS, hardware float
- probe-rs replaces older tools like OpenOCD + GDB
- The micro:bit has a built-in debug probe (no extra hardware!)

**Common student questions:**

- *"What if I don't have a micro:bit?"* - You can use QEMU for some examples,
  but hardware is recommended.
- *"Why `thumbv7em-none-eabihf`?"* - `thumb` = instruction set, `v7em` = Cortex-M4,
  `none` = no OS, `eabihf` = embedded ABI with hardware floating-point.
- *"Can I use OpenOCD instead of probe-rs?"* - Yes, but probe-rs is simpler to
  set up and is the modern standard for Rust embedded development.

</details>
