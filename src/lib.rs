//! # Slint Macro Examples for rust-analyzer Testing
//!
//! This crate contains a curated collection of `slint!` macro examples designed for:
//! - Profiling rust-analyzer's proc-macro-srv performance
//! - Testing proc-macro expansion edge cases
//! - Reproducing known issues (#685 dash identifiers, #16512 performance)
//!
//! ## Usage
//!
//! ```bash
//! # Profile macro expansion
//! rust-analyzer analysis-stats . --source-root-only
//!
//! # Build to test proc-macro expansion
//! cargo check
//! ```

// === Baseline & Basic ===
pub mod minimal;
pub mod simple_ui;

// === Language Features ===
pub mod bindings;
pub mod callbacks;
pub mod functions;
pub mod globals;
pub mod layouts;
pub mod models;
pub mod std_widgets;

// === Issue Reproduction ===
pub mod dash_identifiers;

// === Size Variations (for profiling) ===
pub mod size_large;
pub mod size_medium;
pub mod size_small;
pub mod size_xlarge;

// === Stress Tests ===
pub mod stress_deep_nesting;
pub mod stress_many_components;
pub mod stress_many_properties;
