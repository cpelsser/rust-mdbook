# A Simple GUI Library

Let us design a classical GUI library using our new knowledge of traits and
trait objects.

We will have a number of widgets in our library:

* `Window`: has a `title` and contains other widgets.
* `Button`: has a `label` and a callback function which is invoked when the
  button is pressed.
* `Label`: has a `label`.

The widgets will implement a `Widget` trait, see below.

Copy the code below to <https://play.rust-lang.org/>, fill in the missing
`draw_into` methods so that you implement the `Widget` trait:

```rust,should_panic
// TODO: remove this when you're done with your implementation.
#![allow(unused_imports, unused_variables, dead_code)]

{{#include simple-gui.rs:setup}}

{{#include simple-gui.rs:Label-width}}
        unimplemented!()
    }

{{#include simple-gui.rs:Label-draw_into}}
        unimplemented!()
    }
}

{{#include simple-gui.rs:Button-width}}
        unimplemented!()
    }

{{#include simple-gui.rs:Button-draw_into}}
        unimplemented!()
    }
}

{{#include simple-gui.rs:Window-width}}
        unimplemented!()
    }

{{#include simple-gui.rs:Window-draw_into}}
        unimplemented!()
    }
}

{{#include simple-gui.rs:main}}
```

The output of the above program can be something simple like this:

```text
========
Rust GUI Demo 1.23
========

This is a small text GUI demo.

| Click me! |
```

If you want to draw aligned text, you can use the
[fill/alignment](https://doc.rust-lang.org/std/fmt/index.html#fillalignment)
formatting operators. In particular, notice how you can pad with different
characters (here a `'/'`) and how you can control alignment:

```rust,editable
fn main() {
    let width = 10;
    println!("left aligned:  |{:/<width$}|", "foo");
    println!("centered:      |{:/^width$}|", "foo");
    println!("right aligned: |{:/>width$}|", "foo");
}
```

Using such alignment tricks, you can for example produce output like this:

```text
+--------------------------------+
|       Rust GUI Demo 1.23       |
+================================+
| This is a small text GUI demo. |
| +-----------+                  |
| | Click me! |                  |
| +-----------+                  |
+--------------------------------+
```

<details>

**Exercise guidance for speakers:**
- This exercise demonstrates trait objects (`dyn Widget`) and polymorphism.
- `Box<dyn Widget>` allows storing different widget types in the same Vec.
- The `draw_into` method writes to any `Write` impl (testable with String buffer).
- Focus on composition: Window contains other Widgets.

**Key concepts practiced:**
1. Trait definitions with multiple methods.
2. Implementing traits for multiple types.
3. Using trait objects for heterogeneous collections.
4. `Box<dyn Trait>` for owned trait objects.

**Hints to give if stuck:**
- `width()` calculates the widget's width for layout.
- `draw_into` writes the widget's representation to the buffer.
- Use `write!` and `writeln!` macros for formatting.
- Window's width is the max of its children's widths plus borders.

**Discussion points:**
- Why `Box<dyn Widget>` instead of generics?
- Trade-offs of dynamic dispatch vs static dispatch.
- How would you add a callback to Button?

</details>
