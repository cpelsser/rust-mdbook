# Double Frees in Modern C++

Modern C++ solves this differently:

```c++
std::string s1 = "Cpp";
std::string s2 = s1;  // Duplicate the data in s1.
```

* The heap data from `s1` is duplicated and `s2` gets its own independent copy.
* When `s1` and `s2` go out of scope, they each free their own memory.

Before copy-assignment:


```bob
 Stack                             Heap
.- - - - - - - - - - - - - -.     .- - - - - - - - - - - -.
:                           :     :                       :
:    s1                     :     :                       :
:   +-----------+-------+   :     :   +----+----+----+    :
:   | ptr       |   o---+---+--+--+-->| C  | p  | p  |    :
:   | len       |     3 |   :     :   +----+----+----+    :
:   | capacity  |     3 |   :     :                       :
:   +-----------+-------+   :     :                       :
:                           :     `- - - - - - - - - - - -'
`- - - - - - - - - - - - - -'
```

After copy-assignment:

```bob
 Stack                             Heap
.- - - - - - - - - - - - - -.     .- - - - - - - - - - - -.
:                           :     :                       :
:    s1                     :     :                       :
:   +-----------+-------+   :     :   +----+----+----+    :
:   | ptr       |   o---+---+--+--+-->| C  | p  | p  |    :
:   | len       |     3 |   :     :   +----+----+----+    :
:   | capacity  |     3 |   :     :                       :
:   +-----------+-------+   :     :                       :
:                           :     :                       :
:    s2                     :     :                       :
:   +-----------+-------+   :     :   +----+----+----+    :
:   | ptr       |   o---+---+-----+-->| C  | p  | p  |    :
:   | len       |     3 |   :     :   +----+----+----+    :
:   | capacity  |     3 |   :     :                       :
:   +-----------+-------+   :     :                       :
:                           :     `- - - - - - - - - - - -'
`- - - - - - - - - - - - - -'
```

<details>

**Key points for speakers:**
- C++ copies by default — this is safe but can be expensive.
- After assignment, `s1` and `s2` are completely independent.
- This diagram contrasts with Rust's move semantics shown in the previous slide.
- C++ requires explicit `std::move` to get move semantics.

**Common student questions:**
- *"Isn't copying safer?"* - It avoids some bugs but introduces overhead. For large data, implicit copying is a performance trap.
- *"How does Rust avoid the copy?"* - By moving instead. If you need a copy, you explicitly call `.clone()`.
- *"Can C++ do moves like Rust?"* - Yes, with `std::move`, but C++ doesn't prevent use-after-move. The compiler won't stop you from accessing the moved-from object.
- *"Which is better?"* - Rust's approach: safe by default, explicit when you want copies. C++ copies by default, which can be surprising and expensive.

**Comparison point:**
In Rust, `let s2 = s1;` moves. In C++, `std::string s2 = s1;` copies. Different defaults!

</details>
