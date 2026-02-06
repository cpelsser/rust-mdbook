# Paths

Paths are resolved as follows:

1. As a relative path:
   * `foo` or `self::foo` refers to `foo` in the current module,
   * `super::foo` refers to `foo` in the parent module.

2. As an absolute path:
   * `crate::foo` refers to `foo` in the root of the current crate,
   * `bar::foo` refers to `foo` in the `bar` crate.

<details>

`crate::` = crate root, `self::` = current module, `super::` = parent module.

---

**Key points for speakers:**
- `crate::` is the current crate root — like `/` in filesystem paths.
- `self::` is explicitly "current module" — often optional but clarifies intent.
- `super::` goes up one level — like `..` in filesystem paths.
- External crate names are resolved from the extern prelude (dependencies in Cargo.toml).

**Common student questions:**
- *"When should I use absolute vs relative paths?"* - Use `crate::` for items far away in the hierarchy. Use relative paths for nearby items. Consistency matters more than the choice.
- *"What's the difference between `use` and paths?"* - `use` brings items into scope so you don't repeat long paths. Paths are how you reference items.
- *"Can I rename imports?"* - Yes! `use std::io::Result as IoResult;` avoids conflicts.
- *"What if two crates have the same name?"* - Use `extern crate foo as bar;` or rename in Cargo.toml.

**Demo suggestion:**
Show a `use` statement with glob imports (`use std::collections::*;`) and discuss why it's generally discouraged.

</details>
