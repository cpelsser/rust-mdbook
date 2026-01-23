# Unbounded Channels

You get an unbounded and asynchronous channel with `mpsc::channel()`:

```rust,editable
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

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

- `mpsc` stands for "multiple producer, single consumer".
- `send()` returns a `Result` - it fails if the receiver has been dropped.
- The receiver blocks on `recv()` until a message arrives or all senders drop.
- `rx.iter()` iterates until the channel is closed (all senders dropped).
- Unbounded channels can grow without limit, which may cause memory issues.

</details>
