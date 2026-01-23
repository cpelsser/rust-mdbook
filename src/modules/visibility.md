# Visibility

Modules are a privacy boundary:

* Module items are private by default (hides implementation details).
* Parent and sibling items are always visible.

```rust,editable
mod outer {
    fn private() {
        println!("outer::private");
    }

    pub fn public() {
        println!("outer::public");
    }

    mod inner {
        fn private() {
            println!("outer::inner::private");
        }

        pub fn public() {
            println!("outer::inner::public");
            super::private();
        }
    }
}

fn main() {
    outer::public();
}
```

<details>

**Key points for speakers:**
- Private by default is a security-conscious design choice.
- `pub` makes an item visible to parent modules (and their parents, up to `pub` at crate level).
- `super::` accesses the parent module — useful for accessing sibling items.
- Child modules can see private items in parent modules, but not vice versa.

**Common student questions:**
- *"Why can inner call super::private()?"* - Child modules can see their parent's private items. Privacy is about external access, not internal.
- *"What about pub(crate), pub(super)?"* - These are visibility modifiers: `pub(crate)` = visible within crate, `pub(super)` = visible to parent only.
- *"How is this different from Java's package-private?"* - Rust's system is more fine-grained. You can specify exactly which scope can see an item.
- *"Can I access outer::inner::public from main?"* - No! The `inner` module itself isn't `pub`, so you can't reach into it even though `public` is `pub`.

**Demo suggestion:**
Try uncommenting `outer::inner::public()` in main to show the error message about module visibility.

</details>
