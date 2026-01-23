# Bounded Channels

Bounded and synchronous channels make `send` block the current thread:

```rust,editable
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::sync_channel(3);

    thread::spawn(move || {
        let thread_id = thread::current().id();
        for i in 1..10 {
            tx.send(format!("Message {i}")).unwrap();
            println!("{thread_id:?}: sent Message {i}");
        }
        println!("{thread_id:?}: done");
    });
    thread::sleep(Duration::from_millis(100));

    for msg in rx.iter() {
        println!("Main: got {}", msg);
    }
}
```

<details>

- `sync_channel(n)` creates a channel with capacity `n`.
- `send()` blocks if the channel is full until space becomes available.
- Bounded channels provide backpressure, preventing producers from overwhelming
  consumers.
- A capacity of 0 means the sender and receiver must rendezvous (synchronize).
- Compare the output with the unbounded version to see the blocking behavior.

</details>
