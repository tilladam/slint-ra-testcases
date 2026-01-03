//! Dash identifier edge cases (Issue #685).
//!
//! This file tests the `Span::Debug` adjacency detection issue where
//! rust-analyzer differs from rustc in how it handles token spans.
//!
//! In Slint, `foo-bar` (no spaces) is a single hyphenated identifier,
//! while `foo - bar` (with spaces) is a subtraction expression.
//!
//! The macro uses `Span::Debug` output to detect if tokens are adjacent,
//! but rust-analyzer doesn't implement `Debug` for `Span` the same way
//! as rustc, causing parsing differences.
//!
//! See: https://github.com/slint-ui/slint/issues/685

slint::slint! {
    export component DashIdentifiersTest inherits Window {
        // === Hyphenated property names (single tokens) ===

        // Simple hyphenated name
        in property <int> foo-bar;

        // Multiple hyphens
        in property <int> a-b-c-d;

        // Common CSS-like names (avoiding built-in property names)
        in property <color> bg-color;
        in property <length> border-size;
        in property <length> margin-y;
        in property <length> padding-x;
        in property <string> font-name;

        // === Subtraction expressions (multiple tokens) ===

        // Simple subtraction
        in property <int> result1: foo-bar - 10;

        // Subtraction with hyphenated identifier
        in property <int> result2: a-b-c-d - 1;

        // Multiple subtractions
        in property <int> result3: foo-bar - a-b-c-d - 5;

        // === Mixed usage ===

        // Hyphenated name with expression using it
        in property <length> content-width: 100px;
        in property <length> available-width: content-width - border-size - padding-x;

        // === Edge cases ===

        // Property with long hyphenated name
        in property <int> test-value;

        // Multiple properties with similar names
        in property <int> x-pos;
        in property <int> y-pos;
        in property <int> z-pos;

        // Computed property using hyphenated names
        out property <int> total-pos: x-pos + y-pos + z-pos;

        // Subtraction vs hyphenation disambiguation
        in property <int> a;
        in property <int> b;
        in property <int> a-b;  // This is a NEW property, not a - b

        // Expression using both
        out property <int> expr: a-b - a - b;  // (a-b) - a - b

        Rectangle {
            width: content-width;
            height: 100px;
            background: bg-color;
        }
    }

    // Component with many hyphenated callback names
    export component DashCallbacks {
        callback on-click();
        callback on-double-click();
        callback on-mouse-enter();
        callback on-mouse-leave();
        callback on-key-press(string);
        callback on-value-changed(int);
    }

    // Component testing hyphenated struct fields
    export struct Hyphenated-Struct {
        field-one: int,
        field-two: string,
        is-enabled: bool,
        background-color: color,
    }

    export component StructTest {
        in property <Hyphenated-Struct> data: {
            field-one: 42,
            field-two: "test",
            is-enabled: true,
            background-color: #3498db,
        };

        out property <int> extracted: data.field-one - 10;
    }
}
