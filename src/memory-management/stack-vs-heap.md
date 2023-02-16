# The Stack vs The Heap

* Stack: Continuous area of memory for local variables.
  * Values have fixed sizes known at compile time.
  * Extremely fast: just move a stack pointer.
  * Easy to manage: follows function calls.
  * Great memory locality.

* Heap: Storage of values outside of function calls.
  * Values have dynamic sizes determined at runtime.
  * Slightly slower than the stack: some book-keeping needed.
  * No guarantee of memory locality.

<details>

* The stack is used to store the program and the variables of fixed size.
* The heap is used for dynamically allocating memory.

</details>
