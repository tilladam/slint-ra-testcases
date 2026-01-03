//! Functions example.
//!
//! Tests function patterns:
//! - Public functions
//! - Pure functions
//! - Functions with arguments
//! - Functions with return values

slint::slint! {
    export component FunctionsTest inherits Window {
        width: 300px;
        height: 200px;

        in-out property <int> base-value: 10;
        out property <int> computed-value: compute-value(base-value);

        // Public function callable from Rust
        public function set-value(val: int) {
            base-value = val;
        }

        // Public function with return value
        public function get-doubled() -> int {
            return base-value * 2;
        }

        // Pure function for computations
        pure function compute-value(x: int) -> int {
            return x * x + 1;
        }

        // Pure function with multiple arguments
        pure function clamp(value: int, min-val: int, max-val: int) -> int {
            if value < min-val {
                return min-val;
            }
            if value > max-val {
                return max-val;
            }
            return value;
        }

        // Function using other functions
        pure function compute-clamped(x: int) -> int {
            return clamp(compute-value(x), 0, 100);
        }

        Rectangle {
            background: #3498db;

            Text {
                text: "Value: " + computed-value;
                color: white;
            }
        }
    }

    // Component with viewport-related functions
    export component ViewportFunctions {
        in-out property <length> viewport-width;
        in-out property <length> viewport-height;

        public function set-viewport(w: length, h: length) {
            viewport-width = w;
            viewport-height = h;
        }

        pure function area() -> float {
            return (viewport-width / 1px) * (viewport-height / 1px);
        }
    }
}
