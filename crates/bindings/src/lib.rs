//! Property bindings example.
//!
//! Tests various binding patterns:
//! - Simple property bindings
//! - Expression bindings
//! - Two-way bindings
//! - Conditional bindings

slint::slint! {
    export component BindingsTest inherits Window {
        // Input properties
        in property <int> counter: 0;
        in property <string> label: "Default";
        in property <bool> enabled: true;

        // Output properties with bindings
        out property <int> doubled: counter * 2;
        out property <string> display: "Count: " + counter;

        // Two-way binding target
        in-out property <float> slider-value: 0.5;

        // Private property with expression
        private property <color> bg-color: enabled ? #3498db : #95a5a6;

        Rectangle {
            background: bg-color;
            width: 200px;
            height: 100px;

            Text {
                text: display;
                color: white;
            }
        }

        // Binding with ternary operator
        Rectangle {
            y: 120px;
            width: enabled ? 200px : 100px;
            height: 50px;
            background: counter > 10 ? #dd4c3c : #22cc71;
        }

        // Binding with math expressions
        Rectangle {
            y: 180px;
            width: (counter * 10px) + 50px;
            height: 30px;
            background: #9b59b6;
        }
    }

    // Component demonstrating two-way binding
    export component TwoWayBindingTest {
        in-out property <int> value <=> inner.value;

        inner := Rectangle {
            property <int> value: 0;
        }
    }
}
