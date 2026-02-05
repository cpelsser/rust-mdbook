# Safe FFI Wrapper

Rust has great support for calling functions through a _foreign function
interface_ (FFI). We will use this to build a safe wrapper for the `libc`
functions you would use from C to read the filenames of a directory.

You will want to consult the manual pages:

* [`opendir(3)`](https://man7.org/linux/man-pages/man3/opendir.3.html)
* [`readdir(3)`](https://man7.org/linux/man-pages/man3/readdir.3.html)
* [`closedir(3)`](https://man7.org/linux/man-pages/man3/closedir.3.html)

You will also want to browse the [`std::ffi`] module, particularly for [`CStr`]
and [`CString`] types which are used to hold NUL-terminated strings coming from
C. The [Nomicon] also has a very useful chapter about FFI.

[`std::ffi`]: https://doc.rust-lang.org/std/ffi/
[`CStr`]: https://doc.rust-lang.org/std/ffi/struct.CStr.html
[`CString`]: https://doc.rust-lang.org/std/ffi/struct.CString.html
[Nomicon]: https://doc.rust-lang.org/nomicon/ffi.html

Copy the code below to <https://play.rust-lang.org/> and fill in the missing
functions and methods:

```rust,should_panic
// TODO: remove this when you're done with your implementation.
#![allow(unused_imports, unused_variables, dead_code)]

{{#include safe-ffi-wrapper.rs:ffi}}

{{#include safe-ffi-wrapper.rs:DirectoryIterator}}
        unimplemented!()
    }
}

{{#include safe-ffi-wrapper.rs:Iterator}}
        unimplemented!()
    }
}

{{#include safe-ffi-wrapper.rs:Drop}}
        unimplemented!()
    }
}

{{#include safe-ffi-wrapper.rs:main}}
```

<details>

**Exercise guidance for speakers:**
- This is an advanced exercise combining FFI, unsafe, and RAII.
- The wrapper provides a safe Rust interface to unsafe C functions.
- Implements `Iterator` for ergonomic directory traversal.
- `Drop` ensures `closedir` is always called (RAII pattern).

**Key concepts practiced:**
1. FFI declarations with `extern "C"`.
2. Working with raw pointers and null checks.
3. Converting between Rust strings and C strings (`CStr`, `CString`).
4. Implementing standard traits (`Iterator`, `Drop`).
5. Encapsulating unsafe code in safe abstractions.

**Hints to give if stuck:**
- `CString::new(path)?.as_ptr()` converts Rust string to C string.
- Check for null pointer from `opendir` (indicates error).
- `readdir` returns null when no more entries.
- `CStr::from_ptr(dirent.d_name.as_ptr())` converts C string back.
- Implement `Drop` to call `closedir`.

**Safety discussion:**
- Why is the FFI call unsafe?
- How does the wrapper make it safe?
- What invariants does the wrapper maintain?

**Extension ideas:**
- Add error handling with `std::io::Error`.
- Filter out `.` and `..` entries.
- Return `PathBuf` instead of `String`.

</details>
