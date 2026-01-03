//! Minimal macro example - baseline for performance comparison.
//!
//! This is the simplest possible `slint!` macro invocation.
//! Use this as a baseline when profiling macro expansion time.

slint::slint! {
    export component Empty {}
}
