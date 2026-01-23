# Closures

Closures (also known as lambda expressions) are anonymous functions that can
capture values from their environment.

<details>

- Closures have a unique, anonymous type - you can't name it directly.
- Use `impl Fn(...)` or `dyn Fn(...)` to accept closures as arguments.
- Closures that don't capture anything can coerce to function pointers (`fn`).
- The compiler infers whether a closure is `Fn`, `FnMut`, or `FnOnce` based on
  how it uses captured variables.

</details>
