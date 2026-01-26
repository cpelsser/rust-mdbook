# Bare Metal Basics

Before we start programming the micro:bit, let's understand what makes bare metal
Rust different from regular Rust.

When you write a normal Rust program, it runs on top of an operating system that
provides:

- Memory allocation (heap)
- File system access
- Network stack
- Threads and synchronization
- Standard input/output

On bare metal, **none of this exists**. Your program runs directly on the CPU,
and you must handle everything yourself (or use libraries that do).

This section covers:

- [What is `no_std`?](bare-metal/no-std.md) - What you keep and what you lose
- [The Embedded Ecosystem](bare-metal/ecosystem.md) - PACs, HALs, and BSPs

<details>

**Key points:**

- Bare metal doesn't mean "hard" - it means "close to hardware"
- You trade convenience for control and predictability
- The Rust embedded ecosystem provides excellent abstractions

</details>
