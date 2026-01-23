# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is an **mdBook-based Rust programming course** adapted from Google's Comprehensive Rust course. It consists of 7 progressive courses teaching Rust fundamentals through advanced topics, hosted at https://cpelsser.github.io/rust-mdbook/.

## Common Commands

### Building the Book
```bash
mdbook build              # Build the book to ./book/
mdbook serve              # Serve locally with live reload (http://localhost:3000)
```

### Working with Exercises
```bash
# Run a specific exercise binary
cargo run --bin for-loops
cargo run --bin book-library
cargo run --bin points-polygons
cargo run --bin luhn
cargo run --bin strings-iterators
cargo run --bin safe-ffi-wrapper
cargo run --bin simple-gui
cargo run --bin dining-philosophers
cargo run --bin link-checker

# Build all exercises
cargo build -p comprehensive-rust

# Run tests for exercises
cargo test -p comprehensive-rust
```

### Code Formatting
```bash
cargo fmt                 # Format code (max_width = 90)
```

## Architecture

### Workspace Structure
The Cargo workspace has two members:
- `i18n-helpers/` - Internationalization tools for mdBook (paragraph extraction, gettext support)
- `src/exercises/` - Runnable Rust exercise binaries organized by day

### Content Organization
Course content lives in `src/` as Markdown files:
- `src/SUMMARY.md` - mdBook table of contents (defines navigation structure)
- `src/course-1/` through `src/course-7/` - Course-specific content
- `src/exercises/course-1/` through `src/exercises/course-7/` - Exercise files (.md and .rs)
- Topic directories: `basic-syntax/`, `ownership/`, `memory-management/`, `error-handling/`, `traits/`, `generics/`, `closures/`, `iterators/`, `concurrency/`, `testing/`, `unsafe/`, `std/`, `modules/`

### mdBook Configuration
- `book.toml` - Main configuration (preprocessors: links, index, svgbob)
- `theme/index.hbs` - Custom HTML template
- `speaker-notes.js/css` - Presentation mode support
- `svgbob.css` - ASCII-to-SVG diagram styling

## Key Dependencies

- **mdbook** (0.4.21) - Documentation generator
- **mdbook-svgbob** - Converts ASCII art diagrams to SVG
- **reqwest** - HTTP client (used in link-checker exercise)
- **scraper** - HTML parsing (used in link-checker exercise)
- **thiserror** - Error handling derive macros

## CI/CD

GitHub Actions (`.github/workflows/deploy.yml`) automatically builds and deploys to GitHub Pages on push to main.

## Speaker Notes

All content files include comprehensive speaker notes in `<details>` blocks. When adding or editing content:

### Speaker Notes Format
```markdown
<details>

**Key points for speakers:**
- Main concept 1
- Main concept 2

**Common student questions:**
- *"Question?"* - Answer explaining the concept.

**Demo suggestions:** (optional)
- Interactive example to show

</details>
```

### Guidelines for Speaker Notes
- Include 3-5 key points that speakers should emphasize
- Anticipate 2-4 common student questions with clear answers
- Add demo suggestions for complex topics
- Keep answers concise but informative
- Focus on "why" not just "what"

## Content Guidelines

- **Exercises**: Located in `src/exercises/course-N/` with both `.md` (instructions) and `.rs` (solution) files
- **Code examples**: Use `rust,editable` fence for interactive examples, `rust,compile_fail` for intentional errors
- **Diagrams**: Use svgbob code fences for ASCII-art diagrams (automatically converted to SVG)
- **Cross-references**: Link to related sections using relative markdown paths

## Exercise Binaries

Available exercise binaries (run with `cargo run --bin <name>`):
- Course 1: `for-loops`, `fibonacci`, `collatz`
- Course 2: `book-library`, `builder-type`
- Course 3: `points-polygons`, `expression-eval`
- Course 4: `luhn`, `strings-iterators`, `iterator-chain`
- Course 5: `simple-gui`, `log-filter`
- Course 6: `safe-ffi-wrapper`
- Course 7: `dining-philosophers`, `link-checker`
