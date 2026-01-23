# Scope-Based Memory Management

Constructors and destructors let you hook into the lifetime of an object.

By wrapping a pointer in an object, you can free memory when the object is
destroyed. The compiler guarantees that this happens, even if an exception is
raised.

This is often called _resource acquisition is initialization_ (RAII) and gives
you smart pointers.

## C++ Example

```c++
void say_hello(std::unique_ptr<Person> person) {
  std::cout << "Hello " << person->name << std::endl;
}
```

* The `std::unique_ptr` object is allocated on the stack, and points to
  memory allocated on the heap.
* At the end of `say_hello`, the `std::unique_ptr` destructor will run.
* The destructor frees the `Person` object it points to.

Special move constructors are used when passing ownership to a function:

```c++
std::unique_ptr<Person> person = find_person("Carla");
say_hello(std::move(person));
```

<details>

**Key points for speakers:**
- RAII = "Resource Acquisition Is Initialization" — resources tied to object lifetime.
- This C++ pattern is the foundation for Rust's ownership model.
- In C++, RAII is a convention. In Rust, it's enforced by the compiler.
- `std::move` in C++ is similar to Rust's move semantics, but C++ doesn't prevent use-after-move.

**Common student questions:**
- *"What if I forget std::move in C++?"* - It might compile but do the wrong thing (copy instead of move, or compile error depending on type).
- *"How is Rust different?"* - In Rust, moves are the default, and the compiler prevents use-after-move.
- *"What resources besides memory use RAII?"* - File handles, network sockets, mutex locks, database connections — anything that needs cleanup.

**Teaching tip:**
This slide sets up *why* Rust's approach is valuable. Emphasize that C++ has the right idea but can't fully enforce it.

</details>
