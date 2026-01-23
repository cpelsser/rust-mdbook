# Rust Programming: Course Series

Welcome to the **Rust Programming Course Series**, a comprehensive journey through mastering Rust, one of the most powerful and reliable systems programming languages available today. This series consists of seven well-structured courses (~2 hours each), designed to progressively build your understanding and skills in Rust.

## Courses Overview

### Course 1: Introduction to Rust
Learn the fundamentals of Rust, including syntax, basic data types, and functions.
- **Key Topics**: Hello World, Why Rust, Scalar/Compound Types, References, Slices, Functions
- **Exercises**: Implicit Conversions, Matrix Multiplication, For Loops, Palindrome, Fibonacci, Collatz Sequence

### Course 2: Ownership and Memory Safety
Dive deep into Rust's unique ownership model, enabling memory safety without garbage collection.
- **Key Topics**: Variables, Type Inference, Memory Management (Stack/Heap), Ownership, Move Semantics, Borrowing, Lifetimes
- **Exercises**: Book Library, Iterators and Ownership, Builder Type

### Course 3: Data Structures and Pattern Matching
Master Rust's powerful type system with structs, enums, and pattern matching.
- **Key Topics**: Structs, Tuple Structs, Enums, Methods, Pattern Matching, Destructuring, Match Guards
- **Exercises**: Health Statistics, Points and Polygons, Expression Evaluation

### Course 4: Standard Library and Iterators
Explore Rust's standard library collections and the power of iterators.
- **Key Topics**: Control Flow, Option/Result, String, Vec, HashMap, Box, Rc, Iterators, Modules
- **Exercises**: Luhn Algorithm, Strings and Iterators, Iterator Method Chaining

### Course 5: Traits, Generics, and Closures
Learn Rust's approach to polymorphism and functional programming.
- **Key Topics**: Traits, Deriving, Generic Types, Trait Bounds, impl Trait, Closures (Syntax, Capturing, Fn traits), Trait Objects
- **Exercises**: Simple GUI Library, Log Filter

### Course 6: Error Handling, Testing, and Unsafe Rust
Build robust applications with proper error handling and learn about Rust's escape hatches.
- **Key Topics**: Panics, Result, Error Propagation (?), thiserror, Unit/Integration/Doc Tests, Unsafe Rust, FFI
- **Exercises**: Safe FFI Wrapper

### Course 7: Concurrency
Leverage Rust's concurrency model for safe and efficient multi-threading.
- **Key Topics**: Threads, Scoped Threads, Channels, Shared State (Arc, Mutex), Send and Sync traits
- **Exercises**: Dining Philosophers, Multi-threaded Link Checker

## For Instructors

This course includes comprehensive **speaker notes** throughout all content files. Speaker notes are contained in `<details>` blocks and include:

- **Key points for speakers** - Main concepts to emphasize during instruction
- **Common student questions** - Anticipated questions with suggested answers
- **Demo suggestions** - Interactive examples to show during class
- **Teaching tips** - Advice for effective instruction

Speaker notes are designed for instructors who may be new to teaching Rust, providing guidance on pacing, common pitfalls, and how to address student confusion.

## How to Use This Course

1. **Access the Material**: Visit [https://cpelsser.github.io/rust-mdbook/](https://cpelsser.github.io/rust-mdbook/) to access the courses.
2. **Progress Sequentially**: Each course builds upon the previous one, so it is recommended to follow the order.
3. **Practice Regularly**: Rust is best learned through hands-on coding. Use the exercises provided in each course.

## Building Locally

### HTML Book

```bash
# Install mdbook and dependencies
cargo install mdbook@0.4.52 mdbook-svgbob

# Build the book
mdbook build

# Serve locally with live reload
mdbook serve
```

### PDF Book

Building the PDF requires additional dependencies:

#### 1. Install Tools

```bash
# mdbook and mdbook-pandoc (specific versions required)
cargo install mdbook@0.4.52 mdbook-svgbob mdbook-pandoc@0.10.4

# Pandoc
brew install pandoc

# LaTeX (MacTeX or BasicTeX)
brew install --cask mactex
# Or for a smaller install: brew install --cask basictex
```

#### 2. Install LaTeX Packages

```bash
sudo tlmgr install lualatex-math selnolig
```

#### 3. Install Fonts

```bash
brew install --cask font-noto-serif font-noto-sans font-noto-sans-mono font-noto-color-emoji font-noto-sans-math
```

#### 4. Build the PDF

```bash
MDBOOK_OUTPUT__PANDOC__DISABLED=false mdbook build
```

The PDF will be generated at `book/pandoc/pdf/linfo2315-rust-course.pdf`.

#### Version Compatibility

| Tool | Version |
|------|---------|
| mdbook | 0.4.52 |
| mdbook-pandoc | 0.10.4 |
| mdbook-svgbob | 0.2.2 |
| Pandoc | 3.x |

## Prerequisites
- Familiarity with programming concepts (preferred but not required).
- A computer with Rust installed ([Get Started with Rust](https://www.rust-lang.org/learn/get-started)).

## Acknowledgments

This course was adapted from the excellent [Comprehensive Rust course](https://google.github.io/comprehensive-rust/) by Google.

## Contribution
Contributions to the course materials or feedback are welcome! If you encounter issues or have suggestions, please open an issue or submit a pull request on the [GitHub repository](https://github.com/cpelsser/rust-mdbook).

---

Happy coding!
