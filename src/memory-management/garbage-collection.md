# Automatic Memory Management

An alternative to manual and scope-based memory management is automatic memory
management:

* The programmer never allocates or deallocates memory explicitly.
* A garbage collector finds unused memory and deallocates it for the programmer.

## Java Example

The `person` object is not deallocated after `sayHello` returns:

```java
void sayHello(Person person) {
  System.out.println("Hello " + person.getName());
}
```

<details>

A garbage collector automatically finds and frees unused memory at runtime.

---

**Key points for speakers:**
- GC languages trade predictability for convenience — you don't manage memory, but can't control when cleanup happens.
- GC pauses can be problematic for real-time systems (games, audio, trading).
- Java, Go, Python, JavaScript, C# all use garbage collection.
- Rust chose a different path: compile-time memory management.

**Common student questions:**
- *"Why doesn't Rust use GC?"* - GC has runtime overhead and unpredictable pauses. Rust targets systems programming where these matter.
- *"Is GC always bad?"* - No! For many applications, GC is fine. But Rust's approach gives you GC-like safety without GC's costs.
- *"Can I use GC in Rust?"* - There are crate options, but idiomatic Rust doesn't need them.
- *"What about reference counting?"* - `Rc` and `Arc` are a form of automatic memory management, but deterministic (no pauses).

</details>
