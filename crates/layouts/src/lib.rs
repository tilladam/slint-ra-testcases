//! Layout patterns example.
//!
//! Tests various layout constructs:
//! - VerticalLayout
//! - HorizontalLayout
//! - GridLayout
//! - Layout properties (spacing, padding, alignment)

slint::slint! {
    export component LayoutsTest inherits Window {
        width: 600px;
        height: 400px;

        HorizontalLayout {
            spacing: 10px;
            padding: 20px;

            // Left panel with vertical layout
            VerticalLayout {
                spacing: 5px;
                alignment: start;

                Rectangle { background: #dd4c3c; height: 40px; }
                Rectangle { background: #e67e22; height: 40px; }
                Rectangle { background: #f1c40f; height: 40px; }
            }

            // Center panel with grid layout
            GridLayout {
                spacing: 5px;

                Row {
                    Rectangle { background: #1abc9c; }
                    Rectangle { background: #16a085; }
                }
                Row {
                    Rectangle { background: #22cc71; }
                    Rectangle { background: #27ae60; }
                }
                Row {
                    Rectangle { background: #3498db; colspan: 2; }
                }
            }

            // Right panel with nested layouts
            VerticalLayout {
                spacing: 10px;
                alignment: center;

                HorizontalLayout {
                    spacing: 5px;
                    Rectangle { background: #9b59b6; width: 30px; height: 30px; }
                    Rectangle { background: #8e44ad; width: 30px; height: 30px; }
                }

                HorizontalLayout {
                    spacing: 5px;
                    Rectangle { background: #34495a; width: 30px; height: 30px; }
                    Rectangle { background: #2c3e50; width: 30px; height: 30px; }
                }
            }
        }
    }

    // Component with stretch factors
    export component StretchTest {
        HorizontalLayout {
            Rectangle { background: #dd4c3c; horizontal-stretch: 1; }
            Rectangle { background: #3498db; horizontal-stretch: 2; }
            Rectangle { background: #22cc71; horizontal-stretch: 1; }
        }
    }
}
