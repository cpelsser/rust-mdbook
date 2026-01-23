# Dining Philosophers

The dining philosophers problem is a classic problem in concurrency:

> Five philosophers dine together at the same table. Each philosopher has their
> own place at the table. There is a fork between each plate. The dish served is
> a kind of spaghetti which has to be eaten with two forks. Each philosopher can
> only alternately think and eat. Moreover, a philosopher can only eat their
> spaghetti when they have both a left and right fork. Thus two forks will only
> be available when their two nearest neighbors are thinking, not eating. After
> an individual philosopher finishes eating, they will put down both forks.

You will need a local [Cargo installation](../../cargo/running-locally.md) for
this exercise. Copy the code below to `src/main.rs` file, fill out the blanks,
and test that `cargo run` does not deadlock:

```rust,compile_fail
{{#include dining-philosophers.rs:Philosopher}}
    // left_fork: ...
    // right_fork: ...
    // thoughts: ...
}

{{#include dining-philosophers.rs:Philosopher-think}}

{{#include dining-philosophers.rs:Philosopher-eat}}
        // Pick up forks...
{{#include dining-philosophers.rs:Philosopher-eat-end}}
    // Create forks

    // Create philosophers

    // Make them think and eat

    // Output their thoughts
}
```

<details>

**Exercise guidance for speakers:**
- Classic concurrency exercise demonstrating deadlock and how to avoid it.
- Uses `Arc<Mutex<T>>` for shared state across threads.
- The naive solution (each philosopher picks up left fork, then right) deadlocks!
- Solution: break symmetry (e.g., one philosopher picks up forks in opposite order).

**Key concepts practiced:**
1. Thread spawning with `std::thread::spawn`.
2. Shared state with `Arc<Mutex<T>>`.
3. Deadlock causes and prevention strategies.
4. Channel communication with `mpsc`.

**Deadlock explanation:**
If all philosophers pick up their left fork simultaneously, no one can pick up their right fork. They all wait forever.

**Common solutions:**
1. **Asymmetric**: One philosopher picks up right fork first.
2. **Resource hierarchy**: Number forks, always pick up lower-numbered first.
3. **Arbitrator**: Central authority grants permission to eat.
4. **Try-lock**: If can't get second fork, release first and retry.

**Hints to give if stuck:**
- Forks are `Arc<Mutex<()>>` or `Arc<Mutex<bool>>`.
- Each philosopher is a separate thread.
- Use channels to collect thoughts.
- To avoid deadlock: make one philosopher "left-handed."

**Testing:**
Run multiple times — deadlocks are non-deterministic. If it hangs, there's a deadlock.

</details>
