# Scope-Based Memory Management

The idea of _scope-based memory management_ is to tie resource cleanup to the
lifetime of a variable. When the variable goes out of scope, the resource is
automatically released.

This pattern is often called _resource acquisition is initialization_ (RAII).

## The Problem with Manual Cleanup

In C, you must remember to call `free()` before returning:

```c
void process_data(size_t n) {
    int* data = (int*)malloc(n * sizeof(int));
    // ... process data ...
    if (error_condition) {
        free(data);  // Must remember to free!
        return;
    }
    // ... more processing ...
    free(data);  // Must free here too!
}
```

Every exit path needs cleanup code. This is error-prone.

## Python's Approach: Context Managers

Python solves this for files and resources with the `with` statement:

```python
with open("file.txt") as f:
    data = f.read()
# File is automatically closed here, even if an exception occurred
```

The file is closed automatically when leaving the `with` block, regardless of
how you exit (normal return, exception, etc.).

## Rust's Approach: Automatic Drop

Rust applies this pattern to *all* values automatically:

```rust,editable
fn say_hello(person: Box<Person>) {
    println!("Hello {}", person.name);
}
// person is automatically freed here when it goes out of scope
```

* The `Box<Person>` is allocated on the heap.
* At the end of `say_hello`, `person` goes out of scope.
* Rust automatically calls the destructor (the `Drop` trait) and frees the memory.

No explicit cleanup needed, and the compiler guarantees it happens.

<details>

**Key points for speakers:**
- RAII = "Resource Acquisition Is Initialization" — resources tied to object lifetime.
- Python's `with` statement is a limited form of RAII (only for explicitly managed resources).
- Rust applies this automatically to ALL values — no special syntax needed.
- The compiler enforces this — you can't forget to clean up.

**Common student questions:**
- *"How is Rust different from Python's with?"* - In Python, you must explicitly use `with`. In Rust, ALL values get automatic cleanup. Also, Rust has no garbage collector — cleanup happens immediately when scope ends.
- *"What resources besides memory use this pattern?"* - File handles, network sockets, mutex locks, database connections — anything that needs cleanup.
- *"When does cleanup happen?"* - Immediately when the variable goes out of scope, not later during garbage collection.

**Teaching tip:**
Emphasize that Rust makes the "safe by default" behavior automatic, unlike Python where you must remember to use `with`.

</details>
