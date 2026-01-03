//! Models and for-loops example.
//!
//! Tests model-related patterns:
//! - Struct definitions
//! - Array properties
//! - for-in loops
//! - Model indexing

slint::slint! {
    // Struct definitions
    export struct Item {
        name: string,
        value: int,
        enabled: bool,
    }

    export struct Point {
        x: float,
        y: float,
    }

    export struct ColorItem {
        color: color,
        label: string,
    }

    export component ModelsTest inherits Window {
        width: 400px;
        height: 300px;

        // Array property with struct type
        in property <[Item]> items: [
            { name: "First", value: 1, enabled: true },
            { name: "Second", value: 2, enabled: false },
            { name: "Third", value: 3, enabled: true },
        ];

        // Simple array
        in property <[string]> labels: ["One", "Two", "Three"];

        // Color array
        in property <[ColorItem]> colors: [
            { color: #dd4c3c, label: "Red" },
            { color: #3498db, label: "Blue" },
            { color: #22cc71, label: "Green" },
        ];

        VerticalLayout {
            spacing: 5px;
            padding: 10px;

            // For loop with struct items
            for item[index] in items: Rectangle {
                height: 30px;
                background: item.enabled ? #3498db : #95a5a6;

                HorizontalLayout {
                    Text { text: index + ": "; }
                    Text { text: item.name; }
                    Text { text: " = " + item.value; }
                }
            }

            // For loop with colors
            HorizontalLayout {
                spacing: 5px;

                for c in colors: Rectangle {
                    width: 50px;
                    height: 50px;
                    background: c.color;

                    Text {
                        text: c.label;
                        color: white;
                        font-size: 10px;
                    }
                }
            }
        }
    }
}
