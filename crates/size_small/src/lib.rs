//! Small size example (~20 lines of Slint).
//!
//! Use this for baseline performance measurements.
//! Compare expansion time against medium, large, and xlarge.

slint::slint! {
    export component SmallComponent inherits Window {
        width: 300px;
        height: 200px;

        in property <string> label: "Small";
        in property <int> counter: 0;

        callback clicked();

        Rectangle {
            background: #3498db;

            Text {
                text: label + ": " + counter;
                color: white;
            }

            TouchArea {
                clicked => { root.clicked(); }
            }
        }
    }
}
