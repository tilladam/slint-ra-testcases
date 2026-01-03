# Slint Macro Examples for rust-analyzer Testing

A curated collection of `slint!` macro examples designed for testing and profiling rust-analyzer's proc-macro handling.

## Purpose

This repository provides test cases for:

1. **Profiling rust-analyzer** - Various sizes and complexity levels for performance testing
2. **Testing proc-macro-srv** - Edge cases and patterns that exercise the macro server
3. **Reproducing known issues** - Specifically:
   - [#685](https://github.com/slint-ui/slint/issues/685) - Dash identifier parsing (`foo-bar` vs `foo - bar`)
   - [#16512](https://github.com/rust-lang/rust-analyzer/issues/16512) - rust-analyzer performance with Slint

## Structure

```
src/
├── lib.rs                      # Module declarations

# Baseline & Basic
├── minimal.rs                  # Empty component (baseline)
├── simple_ui.rs                # Rectangle, Text, basic properties

# Language Features
├── callbacks.rs                # Callback patterns (args, return values, aliases)
├── bindings.rs                 # Property bindings, expressions, two-way
├── layouts.rs                  # VerticalLayout, HorizontalLayout, GridBox
├── models.rs                   # for-in loops, struct types, [T] arrays
├── std_widgets.rs              # Import from std-widgets.slint
├── functions.rs                # Public functions, pure functions
├── globals.rs                  # Global singletons

# Issue Reproduction
├── dash_identifiers.rs         # foo-bar vs foo - bar (Issue #685)

# Size Variations (for profiling)
├── size_small.rs               # ~20 lines of Slint
├── size_medium.rs              # ~50 lines of Slint
├── size_large.rs               # ~100 lines of Slint
├── size_xlarge.rs              # ~200+ lines of Slint

# Stress Tests
├── stress_deep_nesting.rs      # 10+ levels of nested elements
├── stress_many_properties.rs   # 50+ properties on one component
└── stress_many_components.rs   # 20+ component definitions
```

## Usage

### Basic Compilation Test

```bash
cargo check
```

### Profile Macro Expansion with rust-analyzer

```bash
# From rust-analyzer repository, point to this project
rust-analyzer analysis-stats /path/to/slint-ra-testcases --source-root-only

# Or with more detail
rust-analyzer analysis-stats /path/to/slint-ra-testcases --source-root-only -v
```

### Test Specific Modules

```bash
# Check only specific files for faster iteration
cargo check --lib 2>&1 | grep -E "(error|warning)"
```

### IDE Testing

Open this project in VS Code or another editor with rust-analyzer to observe:

- Macro expansion latency
- Diagnostics accuracy within `slint!` blocks
- Hover/completion behavior
- Go-to-definition functionality

## Example Categories

### Dash Identifiers (Issue #685)

The `dash_identifiers.rs` file tests the `Span::Debug` adjacency detection issue:

```rust
slint::slint! {
    export component Test {
        // Hyphenated property name (single token)
        in property <int> foo-bar;
        // Subtraction expression (three tokens)
        in property <int> result: foo-bar - 10;
    }
}
```

rust-analyzer doesn't implement `Debug` for `Span` the same way as rustc, which can cause parsing differences.

### Size Scaling (Issue #16512)

The `size_*.rs` files provide progressively larger examples:

| File | Lines | Purpose |
|------|-------|---------|
| `size_small.rs` | ~20 | Baseline |
| `size_medium.rs` | ~50 | Typical component |
| `size_large.rs` | ~100 | Complex component |
| `size_xlarge.rs` | ~200+ | Stress test |

Use these to measure how macro expansion time scales with code size.

### Stress Tests

- **`stress_deep_nesting.rs`** - 15+ levels of nested elements
- **`stress_many_properties.rs`** - 50+ properties on a single component
- **`stress_many_components.rs`** - 20+ component definitions in one macro

## Related Links

- [Slint](https://slint.dev/) - The Slint UI framework
- [rust-analyzer](https://rust-analyzer.github.io/) - Rust language server
- [Slint Issue #685](https://github.com/slint-ui/slint/issues/685) - Dash identifier parsing
- [rust-analyzer Issue #16512](https://github.com/rust-lang/rust-analyzer/issues/16512) - Performance with Slint

## License

MIT
