//! Callback patterns example.
//!
//! Tests various callback patterns:
//! - Simple callbacks without arguments
//! - Callbacks with arguments
//! - Callbacks with return values
//! - Callback aliases

slint::slint! {
    export component CallbackTest inherits Window {
        // Simple callback without arguments
        callback clicked();

        // Callback with arguments
        callback value-changed(int);

        // Callback with multiple arguments
        callback position-changed(float, float);

        // Callback with return value
        callback compute(int, int) -> int;

        // Callback alias
        callback button-pressed <=> clicked;

        Rectangle {
            background: #ecf0f1;

            TouchArea {
                clicked => {
                    root.clicked();
                }
            }
        }

        // Internal callback invocation
        Rectangle {
            x: 100px;
            background: #3498db;

            TouchArea {
                clicked => {
                    root.value-changed(42);
                    root.position-changed(1.5, 2.5);
                }
            }
        }
    }

    // Component with callback that has return value
    export component Calculator {
        callback add(int, int) -> int;
        callback subtract(int, int) -> int;
        callback multiply(int, int) -> int;

        in property <int> result;
    }
}
