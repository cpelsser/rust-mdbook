# Manual Memory Management

You allocate and deallocate heap memory yourself.

If not done with care, this can lead to crashes, bugs, security vulnerabilities, and memory leaks.

## C Example

You must call `free` on every pointer you allocate with `malloc`:

```c
void foo(size_t n) {
    int* int_array = (int*)malloc(n * sizeof(int));
    //
    // ... lots of code
    //
    free(int_array);
}
```

Memory is leaked if the function returns early between `malloc` and `free`: the
pointer is lost and we cannot deallocate the memory.

<details>

**Key points for speakers:**
- Manual memory management requires perfect discipline — every malloc needs a free.
- Common bugs: use-after-free, double-free, memory leaks, buffer overflows.
- These bugs cause security vulnerabilities (CVEs) in C codebases regularly.
- Error paths make manual management especially error-prone.

**Common student questions:**
- *"Why do people still use C?"* - Legacy code, bare metal/embedded systems, maximum control. But new projects increasingly choose memory-safe languages.
- *"What happens if you forget free?"* - Memory leak. The memory is never returned to the OS until the program exits.
- *"What's use-after-free?"* - Accessing memory after it's been freed. Can cause crashes or security vulnerabilities.
- *"How do programmers avoid these bugs?"* - Strict coding standards, static analysis tools, and lots of testing. But mistakes still happen.

**Real-world impact:**
Microsoft reports ~70% of their security vulnerabilities are memory safety issues. This is why Rust's memory safety without GC is revolutionary.

</details>
